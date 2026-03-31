#![no_main]
#![no_std]

#[macro_use]
mod console;
mod lang_items;
mod sbi;

use core::arch::global_asm;

use crate::sbi::shutdown;

global_asm!(include_str!("entry.asm"));

#[unsafe(no_mangle)]
fn rust_main() {
    clear_bss();
    println!("Hello, world!");
    shutdown(true)
}

/// 因为写的是操作系统的内核，因此，操作系统自己程序的bss段需要自己手动初始化为0，正常操作系统跑的程序由操作系统负责初始化。
#[allow(function_casts_as_integer)]
fn clear_bss() {
    unsafe extern "C" {
        // 这里的start_bss是在链接脚本中声明的地址，在链接时才能获取到。
        // 程序也是在链接之后形成可执行文件，在源码级这里刚接触或有些困惑，源码-> 汇编-> object file。在obj阶段进行整合。
        // end_bss同理
        fn start_bss();
        fn end_bss();
    }
    (start_bss as usize..end_bss as usize).for_each(|x| unsafe {
        // 裸指针操作改值。
        (x as *mut u8).write_volatile(0);
    });
}
