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