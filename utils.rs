pub unsafe fn outb(port: u16, value: u8)
{
    asm!("outb %al, %dx" :: "{dx}" (port), "{al}" (value) :: "volatile" );
}

pub unsafe fn inb(port: u16) -> u8 {
    let ret: u8;
    asm!( "inb %1, %0" : "=a"(ret) : "Nd"(port) );
    return ret;
}

pub unsafe fn io_wait() {
    asm!("jmp 1f\n\t
          1:jmp 2f\n\t
          2:" );
}
