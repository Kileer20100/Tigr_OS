use crate::kernel::drivers::VGA::b8000::scroll::scroll_control;



pub unsafe fn controler_0xb8000_vga(){

    let hello: &'static str = "Hello Tigr_OS";
    scroll_control(&hello);

}