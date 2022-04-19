#![no_std]
#![no_main]

#[allow(unused)]
struct Multiboot {
    magic: i32,
    flags: i32,
    checksum: i32,
}

const ALIGN: i32 = 1 << 0;
const MEMINFO: i32 = 1 << 1;
const MAGIC: i32 = 0x1BADB002;
const FLAGS: i32 = ALIGN | MEMINFO;

#[no_mangle]
#[link_section = ".multiboot"]
static multiboot: Multiboot = Multiboot {
    magic: MAGIC,
    flags: FLAGS,
    checksum: -(MAGIC + FLAGS),
};

#[derive(Copy, Clone)]
#[repr(align(4))]
pub struct Aligned(u8);

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
