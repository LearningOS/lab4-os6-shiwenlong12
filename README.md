# os1
## console.rs模块
本模块实现了 print 和 println 宏。
## entry.asm
分配并使用启动栈,为内核支持函数调用.
## lang_item.rs
实现panic函数，并通过 #[panic_handler] 属性通知编译器用panic函数来对接 panic! 宏。
## linker.ld
这个链接脚本能够调整内核的内存布局。
## logging.rs
本模块利用 log crate 为你提供了日志功能，使用方式见 main.rs.
## main.rs
作为程序的入口地址。
## sbi.rs
基于 SBI 服务完成输出和关机
### 对外接口
  pub fn console_putchar(c: usize)
这个函数可以用来在屏幕上输出一个字符，用户需要给出输出的字符c。
  pub fn console_getchar()
这个函数可以得到一个字符。
  pub fn shutdown()
这个函数用来进行关机服务。

# os2
## 应用程序设计
    user/src/bin/*.rs ：各个应用程序
    user/src/*.rs ：用户库（包括入口函数、初始化函数、I/O 函数和系统调用接口等）
    user/src/linker.ld ：应用程序的内存布局说明。
## 实现批处理操作系统
在os1的基础上增加了以下模块。
## sync模块
  pub struct UPSafeCell<T> 
允许我们在 单核 上安全使用可变全局变量。
## syscall模块
### 对外接口
  pub fn sys_write(fd: usize, buf: *const u8, len: usize) 
将内存中缓冲区中的数据写入文件。
  pub fn sys_exit(exit_code: i32) 
打印退出的应用程序的返回值并同样调用 run_next_app 切换到下一个应用程序。
  pub fn syscall(syscall_id: usize, args: [usize; 3]) 
syscall 函数并不会实际处理系统调用，而只是根据 syscall ID 分发到具体的处理。
## trap模块
### context.rs子模块
  pub struct TrapContext
结构体TrapContext是Trap 上下文，即在 Trap 发生时需要保存的物理资源内容。 
  pub fn app_init_context(entry: usize, sp: usize)
修改其中的 sepc 寄存器为应用程序入口点 entry， sp 寄存器为我们设定的 一个栈指针，并将 sstatus 寄存器的 SPP 字段设置为 User 。为将特权级由U修改为S作准备。
### mod.rs
#### 对外接口
  pub fn trap_handler(cx: &mut TrapContext)
完成trap的分发和处理。
### trap.S子模块
此模块实现了trap上下文的保存与恢复的汇编代码。

## batch.rs模块
此模块的功能是找到并加载应用程序二进制码
### 对外接口
  pub fn print_app_info()
输出app的信息
  pub fn init() 
调用 print_app_info 的时候第一次用到了全局变量 APP_MANAGER ，它也是在这个时候完成初始化；
  pub fn run_next_app() 
加载并运行下一个应用程序。