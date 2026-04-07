    # 定义一个宏，用于保存通用寄存器
    # sd x\n, \n*8(sp) 用于将通用寄存器x\n保存到sp + n × 8的位置上
    .macro SAVE_GP n
        sd x\n, \n*8(sp)
    .endm
    .macro LOAD_GP n
        ld x\n, \n*8(sp)
    .endm


    .section .text
    .globl __alltraps
    .globl __restore
    # 按照2^2字节对齐
    .align 2

__alltraps:
    # 原子交换 sp 和 sscratch 寄存器的值
    # 执行前：sp = 用户栈，sscratch = 内核栈
    # 执行后：sp = 内核栈，sscratch = 用户栈
    csrrw sp, sscratch, sp
    # now sp->kernel stack, sscratch->user stack
    # allocate a TrapContext on kernel stack
    # 开栈 为保存 34 个寄存器值腾出空间（32 个通用寄存器 + sstatus + sepc）
    addi sp, sp, -34*8
    # 以下为保存通用寄存器的内容，除了x\0(恒为 0，无需保存), x\4(线程指针，应用程序不使用)
    # save general-purpose registers
    sd x1, 1*8(sp)
    # skip sp(x2), we will save it later
    sd x3, 3*8(sp)
    # skip tp(x4), application does not use it
    # save x5~x31
    .set n, 5
    .rept 27
        SAVE_GP %n
        .set n, n+1
    .endr

    # 通用寄存器保存结束了下面开始保存CRS
    # we can use t0/t1/t2 freely, because they were saved on kernel stack
    # 读取控制状态寄存器 sstatus 的值，保存到通用寄存器 t0 中。
    # 读取进入陷阱前的 CPU 状态
    csrr t0, sstatus
    csrr t1, sepc
    sd t0, 32*8(sp)
    sd t1, 33*8(sp)
    # read user stack from sscratch and save it on the kernel stack
    csrr t2, sscratch
    sd t2, 2*8(sp)
    # set input argument of trap_handler(cx: &mut TrapContext)
    mv a0, sp
    call trap_handler

__restore:
    # a0属于调用者保存，因此一直是TrapContext的指针，整个trap的返回就是进入trap的逆操作，将数据读出来，然后覆盖寄存器。
    mv sp, a0
    ld t1, 33*8(sp)
    ld t0, 32*8(sp)
    ld t2, 2*8(sp)
    
    csrw sstatus, t0
    csrw sepc, t1
    csrw sscratch, t2

    ld x\1, 1*8(sp)
    ld x\3, 3*8(sp)
    
    .set n, 5
    .rept 27
        LOAD_GP %n
        .set n, n+1
    .endr

    addi sp, sp, 34*8
    csrrw sp, sscratch, sp

    sret