pub fn testVGA() {
    let vga_buffer = 0xb8000 as *mut u8;
    let hello = b"Hello Tigr_OS";

    for (i, &byte) in hello.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0x2F;
        }
    }
}
