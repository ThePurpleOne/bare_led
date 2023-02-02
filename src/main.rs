#![no_std]
#![no_main]

use core::{panic::PanicInfo, arch::asm};

// DATASHEET : https://cs140e.sergio.bz/docs/BCM2837-ARM-Peripherals.pdf

const GPIO_REG_SIZE:   u32 = 4; // Bytes 
const GPIO_CHUNK_SIZE: u32 = 3; // bits

const GPIO_FSEL_BASE:  u32 = 0x3F20_0000;
const GPIO_SET_BASE:   u32 = 0x3F20_001C;
const GPIO_CLR_BASE:   u32 = 0x3F20_0028;
const GPIO_UD_BASE:    u32 = 0x3F20_0094;
const GPIO_UDCLK_BASE: u32 = 0x3F20_0098;


#[derive(PartialEq, Eq, Clone, Copy)]
#[warn(non_camel_case_types)]
pub enum PinMode {  // Datasheet p.92
	Input = 0b000,
	Output,
	AltFunc5,
	AltFunc4,
	AltFunc0,
	AltFunc1,
	AltFunc2,
	AltFunc3,
}

#[derive(PartialEq, Eq)]
pub enum PinState{
	ON,
	OFF,
}


#[derive(PartialEq, Eq, Clone, Copy)]
pub enum PinPullud // Pull up/Down p.101
{
	Neither = 0,
	PullUp,
	PullDown,
}

struct GPIO
{
	pin : u32,
	mode : PinMode,
	state : PinState,
	pull : PinPullud,
}

impl GPIO
{
	pub fn new(pin : u32, mode : PinMode, pull : PinPullud) -> Self
	{
		Self::set_mode(pin, mode);
		Self::set_pull(pin, pull);

		return GPIO{	pin,
						mode,
						state:PinState::OFF,
						pull};
	}

	fn set_mode(pin : u32, mode : PinMode)
	{
		// ! SETUP THE GPIO MODE
		let chunk_nb = pin % 30;
		let fsel_add =  GPIO_FSEL_BASE + (GPIO_REG_SIZE * (pin / 10));

		// Read the old value to avoid changing it
		let mut val : u32;
		unsafe
		{
			val = core::ptr::read_volatile(fsel_add as *mut u32);
		}

		val &= !0b111 << (chunk_nb * GPIO_CHUNK_SIZE); 			// Clear the 3 bits
		val |= (mode as u32) << (chunk_nb * GPIO_CHUNK_SIZE);	// Set them 

		// Write it back
		unsafe
		{
			core::ptr::write_volatile(fsel_add as *mut u32, val);
		}
	}

	fn delay_ticks(ticks: u32)
	{
		for _ in 0..ticks {
			unsafe
			{
				asm!("nop");
			}
		}
	}

	fn set_pull(pin : u32, pull : PinPullud)
	{
		// ! SETUP THE GPIO PULL
		// Set the PULL MODE
		unsafe{	core::ptr::write_volatile(GPIO_UD_BASE as *mut u32, pull as u32);}
		Self::delay_ticks(150);
		
		// Clock the PULL MODE on the pin
		let add_upclk = GPIO_UDCLK_BASE + (GPIO_REG_SIZE * (pin / 32));
		unsafe{core::ptr::write_volatile(add_upclk as *mut u32, 1 << (pin % 32));}
		Self::delay_ticks(150);
		
		// Clear both registers
		unsafe{	core::ptr::write_volatile(GPIO_UD_BASE as *mut u32, 0 as u32);}
		unsafe{	core::ptr::write_volatile(add_upclk as *mut u32, 0 as u32);}
	}

	pub fn on(&mut self)
	{
		let reg_addr = GPIO_SET_BASE + (GPIO_REG_SIZE * self.pin / 32);
		unsafe
		{
			core::ptr::write_volatile(reg_addr as *mut u32, 1 << (self.pin % 32));
		}
		self.state = PinState::ON;
	}
	
	pub fn off(&mut self)
	{
		let reg_addr = GPIO_CLR_BASE + (GPIO_REG_SIZE * (self.pin / 32));
		unsafe
		{
			core::ptr::write_volatile(reg_addr as *mut u32, 1 << (self.pin % 32));
		}
		self.state = PinState::OFF;
	}

	pub fn pull_down(&mut self)
	{
		self.pull = PinPullud::PullDown;
		Self::set_pull(self.pin, self.pull);
	}

	pub fn pull_up(&mut self)
	{
		Self::set_pull(self.pin, self.pull);
		self.pull = PinPullud::PullUp;
	}

	pub fn pull_neither(&mut self)
	{
		Self::set_pull(self.pin, self.pull);
		self.pull = PinPullud::Neither;
	}

}


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
	loop 
	{
		let mut pin = GPIO::new(2, PinMode::Output, PinPullud::Neither);
		pin.on();
		wait(250000);
		pin.off();
		wait(250000);
	}
}

// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! 
{
	loop {}
}
