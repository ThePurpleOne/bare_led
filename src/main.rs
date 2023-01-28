#![no_std]
#![no_main]
use core::{panic::PanicInfo, arch::asm};


const GPIO_REG_SIZE:   u32 = 4; // Bytes 
const GPIO_CHUNK_SIZE: u32 = 3; // bits

const GPIO_FSEL_BASE:  u32 = 0x3F20_0000;
const GPIO_AS_INPUT:   u32 = 0b000;
const GPIO_AS_OUTPUT:  u32 = 0b001;


#[derive(PartialEq, Eq)]
enum PinMode {
	INPUT,
	OUTPUT,
}

#[derive(PartialEq, Eq)]
enum PinState{
	ON,
	OFF,
}

struct GPIO
{
	pin : u32,
	mode : PinMode,
	state : PinState, 
}

impl GPIO
{
	pub fn new(pin : u32, mode : PinMode) -> Self
	{
		// Set 
		GPIO{}
	}

	fn set_mode(gpio : &mut GPIO,  pin : u32, mode : PinMode)
	{
		if pin > 53 {panic!("Undefined pin number")};

		let chunk_nb = pin % 30;
		let fsel_nb  = pin / 10;
		let fsel_add =  GPIO_FSEL_BASE + (GPIO_REG_SIZE * fsel_nb);

		// Read the old value to avoid changing it
		let val : u32;
		
		unsafe{val = core::ptr::read_volatile(fsel_add as *mut u32);}
		
		val &= !0b111 << (chunk_nb * GPIO_CHUNK_SIZE); // Clear the 3 bits
		if mode == PinMode::OUTPUT
		{
			val |= GPIO_AS_OUTPUT << (chunk_nb * GPIO_CHUNK_SIZE);
		}

		// Write it back
		unsafe{core::ptr::write_volatile(fsel_add as *mut u32, val);}

		gpio.mode = mode;
	}

	pub fn on(gpio : &mut GPIO)
	{
		
	}



}


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
			for _ in 0..250000
			{
				asm!("nop");
			}

			// Set GPIO 2 (PIN 3) to LOW
			core::ptr::write_volatile(0x3F200028 as *mut u32, 1 << 2);
			
			// Wait
			for _ in 0..250000
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
