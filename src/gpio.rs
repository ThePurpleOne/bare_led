use crate::ptr;
use core::arch::asm;
use ptr::BASE_MMIO;

// DATASHEET : https://cs140e.sergio.bz/docs/BCM2837-ARM-Peripherals.pdf

#[derive(PartialEq, Eq, Clone, Copy)]
#[allow(dead_code)]
pub enum PinMode
{
    // Datasheet p.92
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
#[allow(dead_code)]
enum PinState
{
    On,
    Off,
}

#[derive(PartialEq, Eq, Clone, Copy)]
#[allow(dead_code)]
// Pull up/Down p.101
pub enum Pull
{
    Neither = 0,
    PullUp,
    PullDown,
}

#[allow(dead_code)]
#[allow(clippy::upper_case_acronyms)]
pub struct GPIO
{
    pin :   u32,
    mode :  PinMode,
    state : PinState,
    pull :  Pull,
}

#[allow(dead_code)]
impl GPIO
{
	const REG_SIZE 			: u32 = 4; // Bytes
	const CHUNK_SIZE 	: u32 = 3; // bits
	const CHUNK_NB 	: u32 = 10; // chunks

	const FSEL_BASE 	: u32 = BASE_MMIO + 0x0020_0000;
	const SET_BASE 	: u32 = BASE_MMIO + 0x0020_001C;
	const CLR_BASE 	: u32 = BASE_MMIO + 0x0020_0028;
	const UD_BASE 		: u32 = BASE_MMIO + 0x0020_0094;
	const UDCLK_BASE 	: u32 = BASE_MMIO + 0x0020_0098;

    pub fn new(pin : u32, mode : PinMode, pull : Pull) -> Self
    {
        Self::set_mode(pin, mode);
        Self::set_pull(pin, pull);

        GPIO { pin,
               mode,
               state : PinState::Off,
               pull }
    }

    fn set_mode(pin : u32, mode : PinMode)
    {
        // ! SETUP THE GPIO MODE
        let chunk_nb = pin % Self::CHUNK_NB;
        let fsel_add = Self::FSEL_BASE + (Self::REG_SIZE * (pin / 10));

        // Read the old value to avoid changing it
        let mut val : u32;

        val = ptr::read(fsel_add);

        val &= !(0b111 << (chunk_nb * Self::CHUNK_SIZE)); // Clear the 3 bits
        val |= (mode as u32) << (chunk_nb * Self::CHUNK_SIZE); // Set them

        // Write it back
        ptr::write(fsel_add, val);
    }

    fn delay_ticks(ticks : u32)
    {
        for _ in 0..ticks
        {
            unsafe {
                asm!("nop");
            }
        }
    }

    fn set_pull(pin : u32, pull : Pull)
    {
        // ! SETUP THE GPIO PULL
        // Set the PULL MODE
        ptr::write(Self::UD_BASE, pull as u32);
        Self::delay_ticks(150);

        // Clock the PULL MODE on the pin
        let add_upclk = Self::UDCLK_BASE + (Self::REG_SIZE * (pin / 32));
        ptr::write(add_upclk, 1 << (pin % 32));
        Self::delay_ticks(150);

        // Clear both registers
        ptr::write(Self::UD_BASE, 0);
        ptr::write(add_upclk, 0);
    }

    pub fn on(&mut self)
    {
        let reg_addr = Self::SET_BASE + (Self::REG_SIZE * self.pin / 32);
        ptr::write(reg_addr, 1 << (self.pin % 32));
        self.state = PinState::On;
    }

    pub fn off(&mut self)
    {
        let reg_addr = Self::CLR_BASE + (Self::REG_SIZE * (self.pin / 32));
        ptr::write(reg_addr, 1 << (self.pin % 32));
        self.state = PinState::Off;
    }

    pub fn pull_down(&mut self)
    {
        self.pull = Pull::PullDown;
        Self::set_pull(self.pin, self.pull);
    }

    pub fn pull_up(&mut self)
    {
        Self::set_pull(self.pin, self.pull);
        self.pull = Pull::PullUp;
    }

    pub fn pull_neither(&mut self)
    {
        Self::set_pull(self.pin, self.pull);
        self.pull = Pull::Neither;
    }
}
