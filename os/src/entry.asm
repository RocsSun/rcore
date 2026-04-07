    # 定义一个.text.entry段
    .section .text.entry
    # 定义了一个全局变量，在链接object文件时，所有的文件对此符号都可见。
    .globl _start

# start其实是一个内存地址，意思从此地址开始，有以下的汇编内容
_start:
    # 将boot_stack_top地址加载到sp寄存器，这里boot_stack_top其实是栈顶
    la sp, boot_stack_top
    # call是一条伪指令，转到rust_main名字的地址，所以主入口函数必须和这里保持一致在rust源码中。
    call rust_main

    .section .bss.stack
    .globl boot_stack_bottom

boot_stack_bottom:
    # 分配了64kib的栈空间
    .space 4096 * 16
    .globl boot_stack_top

boot_stack_top: