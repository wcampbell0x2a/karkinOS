#![no_std]
#![no_main]

pub mod multiboot_v1;

mod vga_buffer;
use vga_buffer::welcome;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    welcome();
    loop {}
}
