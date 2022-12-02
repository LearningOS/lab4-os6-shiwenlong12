use crate::batch::run_next_app;

/// 功能：退出应用程序并将返回值告知批处理系统。
/// 参数：`xstate` 表示应用程序的返回值。
/// 返回值：该系统调用不应该返回。
/// syscall ID：93
pub fn sys_exit(exit_code: i32) -> ! {
    //打印退出的应用程序的返回值并同样调用 run_next_app 切换到下一个应用程序。
    info!("[kernel] Application exited with code {}", exit_code);
    run_next_app()
}
