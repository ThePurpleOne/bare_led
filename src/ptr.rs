// Baby module to abstract the unsafe stuff

pub fn write(addr: u32, data: u32) {
    unsafe {
        core::ptr::write_volatile(addr as *mut u32, data);
    }
}

pub fn read(addr: u32) -> u32 {
    unsafe { core::ptr::read_volatile(addr as *mut u32) }
}
