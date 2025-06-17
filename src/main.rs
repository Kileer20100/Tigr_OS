#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
use core::arch::asm;
use core::panic::PanicInfo;

use crate::kernel::drivers::VGA::testVGA;
mod kernel;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {

    testVGA();
    

    unsafe {
        loop {
            asm!("hlt", options(nostack, nomem));
        }
    }
}
/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}