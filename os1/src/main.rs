#![no_std]
#![no_main]
<<<<<<< HEAD
=======
//在 main.rs 开头加上 #![feature(panic_info_message)] 才能通过 PanicInfo::message 获取报错信息。
>>>>>>> 0fd7a4e (12.01)
#![feature(panic_info_message)]

use log::*;

#[macro_use]
mod console;
mod lang_items;
mod logging;
mod sbi;

<<<<<<< HEAD
core::arch::global_asm!(include_str!("entry.asm"));

fn clear_bss() {
=======
//嵌入entry.asm的汇编代码
//通过 include_str! 宏将同目录下的汇编代码 entry.asm 转化为字符串并通过 global_asm! 宏嵌入到代码中。
core::arch::global_asm!(include_str!("entry.asm"));

///完成对 .bss 段的清零
fn clear_bss() {
    //extern “C” 可以引用一个外部的 C 函数接口
>>>>>>> 0fd7a4e (12.01)
    extern "C" {
        fn sbss();
        fn ebss();
    }
<<<<<<< HEAD
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

=======
    (sbss as usize..ebss as usize).for_each(|a| 
        //Rust 认为对于裸指针的 解引用 (Dereference) 是一种 unsafe 行为。
        unsafe { (a as *mut u8).write_volatile(0) }
    );
}

//通过宏将 rust_main 标记为 #[no_mangle] 以避免编译器对它的名字进行混淆
>>>>>>> 0fd7a4e (12.01)
#[no_mangle]
pub fn rust_main() -> ! {
    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
        fn boot_stack();
        fn boot_stack_top();
    }
    clear_bss();
    logging::init();
    println!("Hello, world!");
    trace!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    debug!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    info!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    warn!(
        "boot_stack [{:#x}, {:#x})",
        boot_stack as usize, boot_stack_top as usize
    );
    error!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    panic!("Shutdown machine!");
}
