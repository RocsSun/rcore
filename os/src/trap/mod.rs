use core::arch::global_asm;

use riscv::register::{
    scause::{self, Exception, Trap},
    stval, stvec,
};

use crate::{batch::run_next_app, syscall::syscall, trap::context::TrapContext};

pub mod context;

global_asm!(include_str!("trap.S"));

#[allow(function_casts_as_integer)]
pub fn init() {
    unsafe extern "C" {
        fn __alltraps();
    }

    unsafe {
        stvec::write(__alltraps as usize, stvec::TrapMode::Direct);
    }
}

#[unsafe(no_mangle)]
pub fn trap_handler(ctx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let s_tval = stval::read();

    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            ctx.sepc += 4;
            ctx.x[10] = syscall(ctx.x[17], [ctx.x[10], ctx.x[11], ctx.x[12]]) as usize
        }
        Trap::Exception(Exception::StoreFault | Exception::StorePageFault) => {
            println!("[kernel] PageFault in application, kernel killed it.");
            run_next_app();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] IllegalInstruction in application, kernel killed it.");
            run_next_app();
        }
        _ => {
            panic!(
                "[kernel] unsurported trap {:?}, stval: {:#x}",
                scause.cause(),
                s_tval
            );
        }
    }
    ctx
}
