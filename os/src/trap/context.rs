use riscv::register::sstatus::{self, SPP, Sstatus};

/// 异常管理上下文
///
/// 当 CPU 执行完一条指令（如 ecall ）并准备从用户特权级 陷入（ Trap ）到 S 特权级的时候，硬件会自动完成如下这些事情：
///
/// - sstatus 的 SPP 字段会被修改为 CPU 当前的特权级（U/S）。
/// - sepc 会被修改为被中断或触发异常的指令的地址。如 CPU 执行 ecall 指令会触发异常，则 sepc 会被设置为 ecall 指令的地址。
/// - scause/stval 分别会被修改成这次 Trap 的原因以及相关的附加信息。
/// - CPU 会跳转到 stvec 所设置的 Trap 处理入口地址，并将当前特权级设置为 S ，然后从Trap 处理入口地址处开始执行。
#[repr(C)]
pub struct TrapContext {
    // 32 个通用寄存器的位宽由 CPU 的位宽决定。这里的“位宽”与内存中的 8 位（1 字节）不是一个概念。内存以 8 位为最小寻址单元，而寄存器位宽指的是 CPU 一次能够整体处理的位数。
    pub x: [usize; 32],
    pub sstatus: Sstatus,
    pub sepc: usize,
}

impl TrapContext {
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }

    pub fn app_init_context(entry: usize, sp: usize) -> Self {
        let mut s_status = sstatus::read();
        s_status.set_spp(SPP::User);

        let mut ctx = Self {
            x: [0; 32],
            sstatus: s_status,
            sepc: entry,
        };
        ctx.set_sp(sp);
        ctx
    }
}
