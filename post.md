Hello, I'm trying to blink an LED in Bare metal Rust. But i'm running in an issue and i don't even know how now where to debug it. When i plug put the SD card and plug the board, the kernel IS detected because the LED doesnt flash 7 times like it does when it cant find the kernel. But the LED flashes Once then stops (Same with blink code and without)...

I'm using the `armv7a-none-eabi` toolchain.
My linker script is:
```c
ENTRY(_start)

SECTIONS
{
    . = 0x8000;
    .text :
    {
        *(.text._start)
        *(.text*)
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
    .ARM.exidx :
    {
        *(.ARM.exidx*)
    }

    . = ALIGN(4096);
    __bss_end = .;
    __bss_size = __bss_end - __bss_start;
    __end = .;
}
```

My main is:
```rust
#![no_std]
#![no_main]

#[no_mangle]
#[link_section = ".text._start"]
pub extern "C" fn _start() -> ! 
{
	unsafe
	{
		core::ptr::write_volatile(0x20200000 as *mut u32, 1 << 24);
		loop 
		{
			core::ptr::write_volatile(0x2020001C as *mut u32, 1 << 8);
			for _ in 0..5000{asm!("nop");}
			core::ptr::write_volatile(0x20200028 as *mut u32, 1 << 8);
			for _ in 0..5000{asm!("nop");}
		}
	}
}
// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! 
{
	loop {}
}
```

The LED addresses are found in https://www.raspberrypi.org/app/uploads/2012/02/BCM2835-ARM-Peripherals.pdf

And when i dump the elf file, i can see the _start **first** in the .text section at address ``0x8000`.

I really don't know if i did something wrong but i don't know how to find the problem