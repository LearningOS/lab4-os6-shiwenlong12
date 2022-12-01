<<<<<<< HEAD
use crate::sbi::shutdown;
use core::panic::PanicInfo;

=======
//！实现panic函数，并通过 #[panic_handler] 属性通知编译器用panic函数来对接 panic! 宏。
use crate::sbi::shutdown;
use core::panic::PanicInfo;

///我们会从 PanicInfo 解析出错位置并打印出来，然后杀死应用程序。
///PanicInfo 的不可变借用作为输入参数，它在核心库中得以保留
>>>>>>> 0fd7a4e (12.01)
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
<<<<<<< HEAD
        println!("Panicked: {}", info.message().unwrap());
=======
        println!("[kernel] Panicked: {}", info.message().unwrap());
>>>>>>> 0fd7a4e (12.01)
    }
    shutdown()
}
