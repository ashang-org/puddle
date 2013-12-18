use core::slice::Slice;
use core::mem::transmute;
use elf;
use stdio;

pub unsafe fn exec(addr: uint) {
    unsafe {
        let slice = Slice {data: (addr as *u8), len: 100};
        let header = elf::read_header(transmute(slice));
        stdio::putc(header.e_ident.ei_mag[1]);
        stdio::putc(header.e_ident.ei_mag[2]);
        stdio::putc(header.e_ident.ei_mag[3]);
    }
    
}
