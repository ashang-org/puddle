#[allow(ctypes)];
#[no_std];
#[feature(macro_rules)];

use super::core::str::as_bytes;
use super::core::slice::iter;
use super::core::iter::Iterator;
use super::core::option::{Some, Option, None};


static VGA_WIDTH  : u16 = 80;
static VGA_HEIGHT : u16 = 24;

pub enum Color {
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

fn range(lo: uint, hi: uint, it: |uint| ) {
    let mut iter = lo;
    while iter < hi {
        it(iter);
        iter += 1;
    }
}

pub unsafe fn clear_screen(background: Color) {
    range(0, 80*25, |i| {
        *((0xb8000 + i * 2) as *mut u16) = make_vgaentry(0, Black, background);
                
    });
}

pub unsafe fn putchar(x: u16, y: u16, c: u8) {
    let idx : uint =  (y * VGA_WIDTH * 2 + x) as uint;
    *((0xb8000 + idx) as *mut u16) = make_vgaentry(c, Black, Yellow);
}

fn make_vgaentry(c: u8, fg: Color, bg: Color) -> u16 {
    let color = fg as u16 | (bg as u16 << 4);
    return c as u16 | (color << 8);
}

pub unsafe fn write(s: &str, x: u16, y: u16) {
    let bytes : &[u8] = as_bytes(s);
    let mut ix = x;
    let mut iy = y;
    let mut i = 0;
    for b in super::core::slice::iter(bytes) {
        putchar(ix, iy, *b);
        if (ix > VGA_WIDTH * 2) {
            ix = ix - VGA_WIDTH * 2;
            iy += 1;
        }
        i += 1;
        ix += 2;
    }
}
