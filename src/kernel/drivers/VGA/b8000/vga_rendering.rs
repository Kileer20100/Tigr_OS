use crate::kernel::drivers::VGA::b8000::{color::vgaColor0x8000::*};

pub fn text_rendering_0xd8000(text: &'static str) {
    let vga_buffer: *mut u8 = 0xb8000 as *mut u8;

    let hello: &[u8] = text.as_bytes();

    for (i, &byte) in hello.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = TEXT;
        }
    }

    
}



