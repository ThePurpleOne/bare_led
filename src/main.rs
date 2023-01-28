#![no_std]
#![no_main]
use core::{panic::PanicInfo, arch::asm};


const GPIO_REG_SIZE:   u32 = 4; // Bytes 
const GPIO_CHUNK_SIZE: u32 = 3; // bits

const GPIO_FSEL_BASE:  u32 = 0x3F20_0000;
const GPIO_AS_INPUT:   u32 = 0b000;
const GPIO_AS_OUTPUT:  u32 = 0b001;

const GPIO_SET_BASE:   u32 = 0x3F20_001C;
const GPIO_CLR_BASE:   u32 = 0x3F20_0028;


#[derive(PartialEq, Eq)]
pub enum PinMode {
	INPUT,
	OUTPUT,
}

#[derive(PartialEq, Eq)]
pub enum PinState{
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
		if pin > 53 {panic!("Undefined pin number")};

		let chunk_nb = pin % 30; // 2 % 30 = 2
		let fsel_nb  = pin / 10; // 2 / 10 = 0
		// 
		let fsel_add =  GPIO_FSEL_BASE + (GPIO_REG_SIZE * fsel_nb);

		// Read the old value to avoid changing it
		let mut val : u32;
		
		unsafe
		{
			val = core::ptr::read_volatile(fsel_add as *mut u32);
		}
		
		val &= !0b111 << (chunk_nb * GPIO_CHUNK_SIZE); // Clear the 3 bits
		if mode == PinMode::OUTPUT
		{
			val |= GPIO_AS_OUTPUT << (chunk_nb * GPIO_CHUNK_SIZE);
		}

		// Write it back
		unsafe
		{
			core::ptr::write_volatile(fsel_add as *mut u32, val);
		}

		return GPIO{	pin:pin,
						mode:mode,
						state:PinState::OFF};
	}

	pub fn on(&mut self)
	{
		let reg_nb   = self.pin / 32;
		let shift    = self.pin % 32;
		let reg_addr = GPIO_SET_BASE + (GPIO_REG_SIZE * reg_nb);
		unsafe
		{
			core::ptr::write_volatile(reg_addr as *mut u32, 1 << shift);
		}
	}

	pub fn off(&mut self)
	{
		let reg_nb   = self.pin / 32;
		let shift    = self.pin % 32;
		let reg_addr = GPIO_CLR_BASE + (GPIO_REG_SIZE * reg_nb);
		unsafe
		{
			core::ptr::write_volatile(reg_addr as *mut u32, 1 << shift);
		}
	}
}

pub fn wait()
{
	for _ in 0..250000 {
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
		let mut pin = GPIO::new(2, PinMode::OUTPUT);
		pin.on();
		wait();
		pin.off();
		wait();
	}
}

// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! 
{
	loop {}
}
