//! sbi
//!
//! 此模块的代码还是在s模式下，sbi_rt的代码同样也在s模式，s->M 只能通过ecall指令。

use sbi_rt::{NoReason, Shutdown, SystemFailure};

pub fn console_putchar(c: usize) {
    #[allow(deprecated)]
    sbi_rt::legacy::console_putchar(c);
}

pub fn shutdown(failure: bool) -> ! {
    if failure {
        sbi_rt::system_reset(Shutdown, NoReason);
    } else {
        sbi_rt::system_reset(Shutdown, SystemFailure);
    }
    unreachable!()
}
