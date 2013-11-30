#[allow(ctypes)];
#[no_std];
//#[no_core];
#[feature(macro_rules)];
use core::mem::transmute;
use core::slice::iter;
use core::iter::Iterator;
use core::option::{Some, Option, None};
mod core;

static VGA_WIDTH  : u16 = 80;
static VGA_HEIGHT : u16 = 24;


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

fn to_bytes<'a>(s: &'a str) -> &'a [u8] { unsafe { transmute(s) } }

fn range(lo: uint, hi: uint, it: |uint| ) {
    let mut iter = lo;
    while iter < hi {
        it(iter);
        iter += 1;
    }
}

unsafe fn clear_screen(background: Color) {
    range(0, 80*25, |i| {
        *((0xb8000 + i * 2) as *mut u16) = make_vgaentry(0, Black, background);
                
    });
}

unsafe fn putchar(x: u16, y: u16, c: u8) {
    let idx : uint =  (y * VGA_WIDTH * 2 + x) as uint;
    *((0xb8000 + idx) as *mut u16) = make_vgaentry(c, Black, Yellow);
}

fn make_vgaentry(c: u8, fg: Color, bg: Color) -> u16 {
    let color = fg as u16 | (bg as u16 << 4);
    return c as u16 | (color << 8);
}

unsafe fn write(s: &str, x: u16, y: u16) {
    let bytes : &[u8] = to_bytes(s);
    let mut ix = x;
    let mut iy = y;
    let mut i = 0;
    for b in core::slice::iter(bytes) {
        putchar(ix, iy, *b);
        if (ix > VGA_WIDTH * 2) {
            ix = ix - VGA_WIDTH * 2;
            iy += 1;
        }
        i += 1;
        ix += 2;
    }
}


#[no_mangle]
pub unsafe fn main() {
    clear_screen(Green);
    write("Hello!aaa", 2, 3);
}
