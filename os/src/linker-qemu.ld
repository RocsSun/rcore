OUTPUT_ARCH(riscv)
ENTRY(_start)
BASE_ADDRESS = 0x80200000;

SECTIONS
{
    #  当前地址设置为 0x80200000
    . = BASE_ADDRESS;
    # 将当前地址复制给start_kernel变量；后边相同
    start_kernel = .;

    start_text = .;
    # .text 段重新组合，其他下同
    .text : {
        # 将entry.asm中设置的.text.entry 放在最前，用于加载设定好的汇编指令。
        *(.text.entry)
        *(.text)
    }

    . = ALIGN(4K);
    end_text = .;

    start_rodata = .;
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }

    . = ALIGN(4K);
    end_rodata = .;

    start_data = .;
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }
    . = ALIGN(4K);
    end_data = .;

    .bss : {
        # 内核栈调整到为初始化全局变量最前边，
        *(.bss.stack)
        # 这里设置bss段的起始地址，和前边的栈分隔开，防止bss段清零是，将前边设置的stack也给清零了，导致程序崩溃。
        start_bss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }
    . = ALIGN(4K);
    end_bss = .;

    end_kernel = .;
    /DISCARD/ : {
        # 这是记录函数调用栈的。
        *(.eh_frame)
    }
}
