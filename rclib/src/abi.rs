//!
//! ABI
//!
//! ABI的实现
//!

use core::arch::asm;

pub(crate) const SYS_WRITE: usize = 64;
pub(crate) const SYS_EXIT: usize = 93;

/// ecall的ABI的实现
/// 内联汇编，实现trap功能，将权限提升至s模式，
/// a0~a6 传递参数寄存器
/// a0 返回值寄存器
/// a7 syscallid
pub(crate) fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    let ret: isize;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") syscall_id,
        )
    };
    ret
}
