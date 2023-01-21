ENTRY(_start)
PROVIDE(__start = 0x8000);

SECTIONS
{
    . = 0x8000;
	.text :
    {
        *(.text._start)
        *(.text)
    }
    . = ALIGN(4096);
    .rodata :
    {
        *(.rodata)
    }
    . = ALIGN(4096);
    .data :
    {
        *(.data)
    }
    . = ALIGN(4096);
    __bss_start = .;
    .bss :
    {
        bss = .;
        *(.bss)
    }
    . = ALIGN(4096); 
    __bss_end = .;
    __bss_size = __bss_end - __bss_start;
    __end = .;
}