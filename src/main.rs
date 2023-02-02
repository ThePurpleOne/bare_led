#![no_std]
#![no_main]

use core::{panic::PanicInfo, arch::asm};
mod gpio;
use gpio::{PinMode, Pull, GPIO};

pub fn wait(nb: u32)
{
	for _ in 0..nb {
		unsafe
		{
			asm!("nop");
		}
	}
}


#[no_mangle]
#[link_section = ".text._start"]
pub extern "C" fn _start() -> ! 
{
	let mut gpio2 = GPIO::new(2, PinMode::Output, Pull::Neither);
	let mut gpio3 = GPIO::new(3, PinMode::Output, Pull::Neither);
	loop 
	{
		gpio3.off();
		gpio2.on();
		wait(500000);
		gpio3.on();
		gpio2.off();
		wait(500000);
	}
}

// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! 
{
	loop {}
}
