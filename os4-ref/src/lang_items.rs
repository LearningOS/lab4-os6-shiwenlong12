<<<<<<< HEAD:os4-ref/src/lang_items.rs
//! The panic handler

=======
//！实现panic函数，并通过 #[panic_handler] 属性通知编译器用panic函数来对接 panic! 宏。
>>>>>>> 0fd7a4e (12.01):os5-1/src/lang_items.rs
use crate::sbi::shutdown;
use core::panic::PanicInfo;

///我们会从 PanicInfo 解析出错位置并打印出来，然后杀死应用程序。
///PanicInfo 的不可变借用作为输入参数，它在核心库中得以保留
#[panic_handler]
/// panic handler
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        println!("[kernel] Panicked: {}", info.message().unwrap());
    }
    shutdown()
}
