use crate::gpio;
use crate::ptr;
use gpio::{PinMode, Pull, GPIO};

// DATASHEET : https://cs140e.sergio.bz/docs/BCM2837-ARM-Peripherals.pdf


const UART_TX 	: u32 = 14; 			// GPIO 14
const UART_RX 	: u32 = 15; 			// GPIO 15
const UART_AUXEN: u32 = 0x3F21_5004; 	// ENABLE (p.9)

// Auxiliary registers (p.8)
const UART_IO: u32 		= 0x3F21_5040; 	// Mini Uart I/O Data 
const UART_IER: u32 	= 0x3F21_5044;	// Mini Uart Interrupt Enable 
const _UART_IIR: u32 	= 0x3F21_5048;	// Mini Uart Interrupt Identify 
const UART_LCR: u32 	= 0x3F21_504C;	// Mini Uart Line Control 
const UART_MCR: u32 	= 0x3F21_5050;	// Mini Uart Modem Control 
const UART_LSR: u32 	= 0x3F21_5054;	// Mini Uart Line Status 
const _UART_MSR: u32 	= 0x3F21_5058;	// Mini Uart Modem Status 
const _UART_SCRATCH: u32 = 0x3F21_505C; 	// Mini Uart Scratch 
const UART_CNTL: u32 	= 0x3F21_5060; 	// Mini Uart Extra Control 
const _UART_STAT: u32 	= 0x3F21_5064; 	// Mini Uart Extra Status 
const UART_BAUD: u32 	= 0x3F21_5068; 	// Mini Uart Baudrate

#[allow(dead_code)]
pub struct UART
{
	baudrate : u32,
	rx : GPIO,
	tx : GPIO
}


#[allow(dead_code)]
impl UART
{
	pub fn new(baudrate : u32) -> Self
	{
		let tx = GPIO::new(UART_TX, PinMode::AltFunc5, Pull::Neither);
		let rx = GPIO::new(UART_RX, PinMode::AltFunc5, Pull::Neither);
		let uart = UART{
			baudrate,
			rx,
			tx, 
		};

		Self::set_baudrate(baudrate);
		Self::setup();

		return uart;
	}

	// Setup the UART (HARD CODED FOR NOW)
	// 8 bits data
	// 1 stop bit
	// no parities
	fn setup()
	{
		// Disable transmiting and receiving to setup (p.16) 
		ptr::write(UART_CNTL, 0);
		
		// Enable transmit and receive interrupts (p.12)
		ptr::write(UART_IER, 0);

		// 8 bits mode (p.14)
		ptr::write(UART_LCR, 0b11);

		// No Ready To Send (p.14)
		ptr::write(UART_MCR, 0);
		
		// Enable UART
		let mut val = ptr::read(UART_AUXEN);
		val |= 1;
		ptr::write(UART_AUXEN, val);

		// RE-Enable transmiting and receiving (p.16) 
		ptr::write(UART_CNTL, 0b11);
	}

	// Set the Baudrate, Hard coded for now...
	fn set_baudrate(_baudrate : u32)
	{
		// Baudrate to 115200 @250Mhz (p.14)
		ptr::write(UART_BAUD, 270);
	}

	pub fn send(&self, data : char)
	{
		
		// Wait for the FIFO availability (p.15) 
		let mut val : u32;
		loop
		{
			val = ptr::read(UART_LSR);
			if (val & 0x20) == 1
			{
				break;
			}
		}

		// Give data to send
		ptr::write(UART_IO, data as u32);
	}

	pub fn read(&self) -> char
	{
		// Wait for data (p.15)
		let mut val : u32;
		loop
		{
			val = ptr::read(UART_LSR);
			if val & 1 == 0
			{
				break;
			}
		}
		ptr::read(UART_LSR);
		return val as u8 as char;
	}
}