use crate::kernel::drivers::VGA::b8000::vga_rendering::text_rendering_0xd8000;


pub unsafe fn scroll_control(text: &'static str){
    let vga: *mut u8 = 0xd8000 as *mut u8;

    let hello: &[u8] = text.as_bytes();
    text_rendering_0xd8000(&text);

    /*for y in 1..25{
        for x in 1..80{
            let src: usize = (y * 80 + x) * 2;
            let dst: usize = ((y - 1) * 80 + x) * 2;

            *vga.add(dst) = *vga.add(src);
            *vga.add(dst + 1) = *vga.add(src + 1);
        }
    }*/

    for y in 0..24{
        for x in 0..79{

        }
    }

    

}