use crate::batch::run_next_app;

pub fn sys_exit(state: i32) -> ! {
    println!("[kernel] application exited with code {}", state);
    run_next_app()
}
