#![no_std]
#![no_main]
use core::panic::PanicInfo;


// All the code bellow this is put in the .text section
// To make sure that the linker script includes the start first
mod boot{
	use core::arch::global_asm;
	global_asm!(".section .text.__start");
}

#[no_mangle]
pub extern "C" fn _start() -> ! 
{
	loop {}
}


// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! 
{
	loop {}
}
