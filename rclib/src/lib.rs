//!
//! rclib
//!
//! rcore os 的lib库，提供os调用接口。
//!

#![no_std]
#![feature(linkage)]

mod abi;
#[macro_use]
pub mod console;
mod lang_items;
mod syscall;

use core::arch::global_asm;

pub use syscall::*;

global_asm!(include_str!("entry.asm"));

/// U模式入口函数
#[unsafe(no_mangle)]
fn rust_lib_start() -> ! {
    clear_bss();
    exit(main());
    panic!("unreachable after sys_exit!");
}

/// 兜底，用户程序没有main的时候，链接此函数
#[linkage = "weak"]
#[unsafe(no_mangle)]
fn main() -> i32 {
    panic!("Cannot find main!");
}

#[allow(function_casts_as_integer)]
fn clear_bss() {
    unsafe extern "C" {
        safe fn start_bss();
        safe fn end_bss();
    }

    (start_bss as usize..end_bss as usize).for_each(|x| unsafe {
        (x as *mut u8).write_volatile(0);
    });
}
