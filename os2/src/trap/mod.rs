mod context;

use crate::batch::run_next_app;
use crate::syscall::syscall;
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Trap},
    stval, stvec,
};

//我们在 os/src/trap/trap.S 中实现 Trap 上下文保存/恢复的汇编代码，
//分别用外部符号 __alltraps 和 __restore 标记为函数，
//并通过 global_asm! 宏将 trap.S 这段汇编代码插入进来。
core::arch::global_asm!(include_str!("trap.S"));

pub fn init() {
    extern "C" {
        fn __alltraps();
    }
    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
}

/// 完成trap的分发和处理
// 返回值为 &mut TrapContext 并将传入的 cx 原样返回，
// 因此在 __restore 的时候 a0 寄存器在调用 trap_handler 前后并没有发生变化，
// 仍然指向分配 Trap 上下文之后的内核栈栈顶，和此时 sp 的值相同。
#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();
    //据 scause 寄存器所保存的 Trap 的原因进行分发处理。
    //这里我们无需手动操作这些 CSR ，而是使用 Rust 第三方库 riscv 。
    match scause.cause() {
        //发现触发 Trap 的原因是来自 U 特权级的 Environment Call，也就是系统调用。
        Trap::Exception(Exception::UserEnvCall) => {
            //修改保存在内核栈上的 Trap 上下文里面 sepc，让其增加 4。
            cx.sepc += 4;
            //用来保存系统调用返回值的 a0 寄存器也会同样发生变化。
            //我们从 Trap 上下文取出作为 syscall ID 的 a7 和系统调用的三个参数 a0~a2 传给 syscall 函数并获取返回值。 
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        //分别处理应用程序出现访存错误和非法指令错误的情形。
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            error!("[kernel] PageFault in application, core dumped.");
            run_next_app();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            error!("[kernel] IllegalInstruction in application, core dumped.");
            run_next_app();
        }
        //当遇到目前还不支持的 Trap 类型的时候，批处理操作系统整个 panic 报错退出。
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
    cx
}

pub use context::TrapContext;
