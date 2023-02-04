#![no_std]
#![no_main]

use core::{panic::PanicInfo, arch::asm};
mod gpio;
mod uart;
mod ptr;
use gpio::{PinMode, Pull, GPIO};
use uart::{UART};

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
	//let uart = UART::new(115200);
	
	loop 
	{
		gpio2.on();
		//uart.send('t');
		wait(250000);
		gpio2.off();
		//uart.send('f');
		wait(250000);
	}
}

// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! 
{
	loop {}
}
