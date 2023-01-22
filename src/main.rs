#![no_std]
#![no_main]
use core::{panic::PanicInfo, arch::asm};


#[no_mangle]
#[link_section = ".text._start"]
pub extern "C" fn _start() -> ! 
{

	// GPIO 2 (PIN 3)
	unsafe
	{
		// Set GPIO 2 (PIN 3) as output
        core::ptr::write_volatile(0x3F200000 as *mut u32, 1 << 6);
		loop 
		{
			// Set GPIO 2 (PIN 3) to HIGH
			core::ptr::write_volatile(0x3F20001C as *mut u32, 1 << 2);
			
			// Wait
			for _ in 0..500000
			{
				asm!("nop");
			}

			// Set GPIO 2 (PIN 3) to LOW
			core::ptr::write_volatile(0x3F200028 as *mut u32, 1 << 2);
			
			// Wait
			for _ in 0..500000
			{
				asm!("nop");
			}
		}
	}
}


// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! 
{
	loop {}
}
