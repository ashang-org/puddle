#[allow(ctypes)];
#[no_std];
#[no_core];

static VGA_WIDTH  : u16 = 80;
static VGA_HEIGHT : u16 = 24;

mod zero;

enum Color {
    Black       = 0,
    Blue        = 1,
    Green       = 2,
    Cyan        = 3,
    Red         = 4,
    Pink        = 5,
    Brown       = 6,
    LightGray   = 7,
    DarkGray    = 8,
    LightBlue   = 9,
    LightGreen  = 10,
    LightCyan   = 11,
    LightRed    = 12,
    LightPink   = 13,
    Yellow      = 14,
    White       = 15,
}

fn range(lo: uint, hi: uint, it: &fn(uint)) {
    let mut iter = lo;
    while iter < hi {
        it(iter);
        iter += 1;
    }
}

unsafe fn clear_screen(background: Color) {
    range(0, 80*25, |i| {
        *((0xb8000 + i * 2) as *mut u16) = make_vgaentry(99, Black, Red);
                
    });
}

unsafe fn putchar(x: u16, y: u16, c: u8) {
    let idx : u16 = y * VGA_WIDTH + x;
    *((0xb8000 + idx) as *mut u16) = make_vgaentry(c, Black, Yellow);
}

fn make_vgaentry(c: u8, fg: Color, bg: Color) -> u16 {
    let color = fg as u16 | (bg as u16 << 4);
    return c as u16 | (color << 8);
}


#[no_mangle]
pub unsafe fn main() {
    clear_screen(White);
    putchar(15, 15, 99);
}
