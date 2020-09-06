#![feature(asm)]
#![no_std]
#![no_main]
extern crate rlibc;

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8; // Convert the address into a raw pointer

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte; // Access memory at the relevant offset and place the byte representing the character
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // The second byte represents the color (cyan in this case)
        }
    }

    loop {}
}
