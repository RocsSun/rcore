#![no_main]
#![no_std]

#[macro_use]
mod console;
mod batch;
mod lang_items;
mod logging;
mod sbi;
mod sync;
mod syscall;
mod trap;

use core::arch::global_asm;

use log::debug;

use crate::logging::init;

global_asm!(include_str!("entry.asm"));

#[allow(function_casts_as_integer)]
#[unsafe(no_mangle)]
fn rust_main() {
    // 手动清零bss
    clear_bss();

    // 初始化日志
    init();

    unsafe extern "C" {
        safe fn start_bss();
        safe fn end_bss();
        safe fn start_text();
        safe fn end_text();
        safe fn start_rodata();
        safe fn end_rodata();
        safe fn start_data();
        safe fn end_data();
        // entry.asm 中定义，开辟了64kib的栈顶和栈底的位置
        safe fn boot_stack_top();
        safe fn boot_stack_bottom();
    }

    debug!(
        "[kernel] .text scetion [{} ~ {}]",
        start_text as usize, end_text as usize
    );
    debug!(
        "[kernel] .rodata scetion [{} ~ {}]",
        start_rodata as usize, end_rodata as usize
    );
    debug!(
        "[kernel] .data scetion [{} ~ {}]",
        start_data as usize, end_data as usize
    );
    debug!(
        "[kernel] .bss scetion [{} ~ {}]",
        start_bss as usize, end_bss as usize
    );
    debug!(
        "[kernel] .stack scetion [{} ~ {}]",
        boot_stack_top as usize, boot_stack_bottom as usize
    );
    println!("Hello, world!");

    trap::init();
    batch::init();
    batch::run_next_app();
}

/// 因为写的是操作系统的内核，因此，操作系统自己程序的bss段需要自己手动初始化为0，正常操作系统跑的程序由操作系统负责初始化。
#[allow(function_casts_as_integer)]
fn clear_bss() {
    unsafe extern "C" {
        // 这里的start_bss是在链接脚本中声明的地址，在链接时才能获取到。
        // 程序也是在链接之后形成可执行文件，在源码级这里刚接触或有些困惑，源码-> 汇编-> object file。在obj阶段进行整合。
        // end_bss同理
        safe fn start_bss();
        safe fn end_bss();
    }
    (start_bss as usize..end_bss as usize).for_each(|x| unsafe {
        // 裸指针操作改值。
        (x as *mut u8).write_volatile(0);
    });
}
