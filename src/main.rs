#![no_std]
#![no_main]

use core::{arch::asm, panic::PanicInfo};
mod gpio;
mod miniuart;
mod ptr;
// use gpio::{PinMode, Pull, GPIO};
use miniuart::Miniuart;

pub fn wait(nb : u32)
{
	for _ in 0..nb
	{
		unsafe {
			asm!("nop");
		}
	}
}

#[no_mangle]
#[link_section = ".text._start"]
pub extern "C" fn _start() -> !
{
	// let mut gpio2 = GPIO::new(2, PinMode::Output, Pull::Neither);
	let muart = Miniuart::new(115200);

	//wait(10000);

	//muart.send_str("-------------------------------------\n");
	//muart.send_str("Welcome to BMRP (Bare metal RASPI) !!\n");
	//muart.send_str("-------------------------------------\n");

	//muart.send_str("> ");
	muart.sendc('!');
	let mut data = muart.readc();
	loop
	{		
		data = muart.readc();
		muart.sendc(data);
	}
}

// This function is called on panic.
#[panic_handler]
fn panic(_info : &PanicInfo) -> !
{
	loop
	{}
}
