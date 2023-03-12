use crate::gpio;
use crate::ptr;
use gpio::{PinMode, Pull, GPIO};
use ptr::BASE_MMIO;

// DATASHEET : https://cs140e.sergio.bz/docs/BCM2837-ARM-Peripherals.pdf

const MUART_TX : u32 = 14; // GPIO 14
const MUART_RX : u32 = 15; // GPIO 15
const MUART_AUXEN : u32 = BASE_MMIO + 0x0021_5004; // ENABLE (p.9)

// Auxiliary registers (p.8)
const MUART_IO : u32 = BASE_MMIO + 0x0021_5040; // Mini Uart I/O Data
const _MUART_IER : u32 = BASE_MMIO + 0x0021_5044; // Mini Uart Interrupt Enable
const MUART_IIR : u32 = BASE_MMIO + 0x0021_5048; // Mini Uart Interrupt Identify
const MUART_LCR : u32 = BASE_MMIO + 0x0021_504C; // Mini Uart Line Control
const _MUART_MCR : u32 = BASE_MMIO + 0x0021_5050; // Mini Uart Modem Control
const MUART_LSR : u32 = BASE_MMIO + 0x0021_5054; // Mini Uart Line Status
const _MUART_MSR : u32 = BASE_MMIO + 0x0021_5058; // Mini Uart Modem Status
const _MUART_SCRATCH : u32 = BASE_MMIO + 0x0021_505C; // Mini Uart Scratch
const MUART_CNTL : u32 = BASE_MMIO + 0x0021_5060; // Mini Uart Extra Control
const _MUART_STAT : u32 = BASE_MMIO + 0x0021_5064; // Mini Uart Extra Status
const MUART_BAUD : u32 = BASE_MMIO + 0x0021_5068; // Mini Uart Baudrate

#[allow(dead_code)]
#[allow(clippy::upper_case_acronyms)]
pub struct Miniuart
{
    baudrate : u32,
    rx :       GPIO,
    tx :       GPIO,
}

#[allow(dead_code)]
impl Miniuart
{
    // Setup the MUART (HARD CODED FOR NOW)
    // 8 bits data
    // 1 stop bit
    // no parities
    pub fn new(baudrate : u32) -> Self
    {
        // Enable MUART
        let mut val = ptr::read(MUART_AUXEN);
        val |= 1;
        ptr::write(MUART_AUXEN, val);

        // Disable transmiting and receiving to setup (p.16)
        ptr::write(MUART_CNTL, 0);

        // Enable transmit and receive interrupts (p.12)
        // ptr::write(MUART_IER, 0);

        // 8 bits mode (p.14)
        ptr::write(MUART_LCR, 0b11);

        // No Ready To Send (p.14)
        // ptr::write(MUART_MCR, 0);

        // Baudrate to 115200 @250Mhz (p.14)
        ptr::write(MUART_BAUD, (250_000_000 / (8 * 115200)) - 1);
		
        // ALTFC5 FOR GPIO 14 and GPIO 15
        let tx = GPIO::new(MUART_TX, PinMode::AltFunc5, Pull::Neither);
        let rx = GPIO::new(MUART_RX, PinMode::AltFunc5, Pull::Neither);
        let uart = Miniuart { baudrate, rx, tx };

        // RE-Enable transmiting and receiving (p.16)
        ptr::write(MUART_CNTL, 0b11);

		// Clear FIFOs
        ptr::write(MUART_IIR, 0b11 << 1);

        uart
    }

    pub fn sendc(&self, data : char)
    {
        // Wait for the FIFO availability (p.15)
        let mut val : u32;
        loop
        {
            val = ptr::read(MUART_LSR);
            if (val & 0x20) != 0
            {
                break;
            }
        }

        ptr::write(MUART_IO, data as u32);
    }

    pub fn send_str(&self, data : &str)
    {
        for char in data.chars()
        {
            //if char == '\n'
            //{
            //    self.sendc('\r');
            //}
            self.sendc(char);
        }
    }

    pub fn readc(&self) -> char
    {
        // Wait for data (p.15)
        let mut val : u32;
        loop
        {
            val = ptr::read(MUART_LSR);
            if (val & 1) != 0
            {
                break;
            }
        }
        ptr::read(MUART_IO) as u8 as char
    }

    // pub fn read_str(delim : char) -> str
    // {

    //     loop{
    //         read
    //     }
    // }
}
