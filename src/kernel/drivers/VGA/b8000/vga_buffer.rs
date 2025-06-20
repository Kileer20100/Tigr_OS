use crate::kernel::drivers::VGA::b8000::color::vgaColor0x8000::*;

pub fn text_key_0xd8000() {
    let vga_buffer = 0xb8000 as *mut u8;
    
    let hello = b"Hello Tigr_OS";

    for (i, &byte) in hello.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = WARNING;
        }
    }
}



