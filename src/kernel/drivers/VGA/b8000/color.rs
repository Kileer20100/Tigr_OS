
#[allow(non_snake_case)]
pub mod vgaColor0x8000{
    pub const BLACK: u8        = 0x00;
    pub const BLUE: u8         = 0x01;
    pub const GREEN: u8        = 0x02;
    pub const CYAN: u8         = 0x03;
    pub const RED: u8          = 0x04;
    pub const MAGENTA: u8      = 0x05;
    pub const BROWN: u8        = 0x06;
    pub const LIGHT_GRAY: u8   = 0x07;
    pub const DARK_GRAY: u8    = 0x08;
    pub const LIGHT_BLUE: u8   = 0x09;
    pub const LIGHT_GREEN: u8  = 0x0A;
    pub const LIGHT_CYAN: u8   = 0x0B;
    pub const LIGHT_RED: u8    = 0x0C;
    pub const PINK: u8         = 0x0D;
    pub const YELLOW: u8       = 0x0E;
    pub const WHITE: u8        = 0x0F;

    pub const fn combine_vga(bacground: u8, foreground: u8) -> u8{
        return (bacground << 4) | foreground;
    }

    pub const ERROR: u8   = combine_vga(LIGHT_RED, RED);
    pub const WARNING: u8 = combine_vga(YELLOW, WHITE);
    pub const INFO: u8    = combine_vga(BLACK, WHITE);
    pub const TEXT: u8    = combine_vga(BLACK, WHITE);

}