#![no_std]
#![no_main]
use core::{panic::PanicInfo, arch::asm};


#[no_mangle]
#[link_section = ".text._start"]
pub extern "C" fn _start() -> ! 
{
	unsafe
	{
		// Set GPIO 14 (PIN 8) as output
		core::ptr::write_volatile(0x20200000 as *mut u32, 1 << 24);
		loop 
		{
			// Set GPIO 14 (PIN 8) to HIGH
			core::ptr::write_volatile(0x2020001C as *mut u32, 1 << 8);
			
			// Wait
			for _ in 0..5000 
			{
				asm!("nop");
			}
			
			// Set GPIO 14 (PIN 8) to LOW
			core::ptr::write_volatile(0x20200028 as *mut u32, 1 << 8);
			
			// Wait
			for _ in 0..5000
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
