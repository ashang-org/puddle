#[allow(ctypes)];
#[no_std];
//#[no_core];
#[feature(macro_rules)];
use super::core::mem;
/* Defines an IDT entry */
struct IDTEntry {
    base_lo: u16,
    sel: u16,        /* Our kernel segment goes here! */
    zero: u8,        /* This will ALWAYS be set to 0! */
    flags: u8,       /* Set using the above table! */
    base_hi: u16
}

/* Defines an IDT pointer */
struct IDTPointer {
    limit: u16,
    base: u32
}

/* Declare an IDT of 256 entries. Although we will only use the
 *  first 32 entries in this tutorial, the rest exists as a bit
 *  of a trap. If any undefined IDT entry is hit, it normally
 *  will cause an "Unhandled Interrupt" exception. Any descriptor
 *  for which the 'presence' bit is cleared (0) will generate an
 *  "Unhandled Interrupt" exception */

#[no_mangle]
static mut idt: [IDTEntry, ..256] = [IDTEntry {base_lo: 0, sel: 0, zero: 0, flags: 0, base_hi: 0}, ..256];

#[no_mangle]
pub static mut idtp: IDTPointer = IDTPointer {limit: 0, base: 0};



/* Use this function to set an entry in the IDT. A lot simpler
*  than twiddling with the GDT ;) */
#[no_mangle]
unsafe fn idt_set_gate(num: u8, f: extern "C" fn(), sel: u16, flags: u8)
{

    let base = f as u32;
    idt[num].sel = sel;
    idt[num].flags = flags;
    idt[num].base_hi = (base >> 16) as u16;
    idt[num].base_lo = (base & (1 << 16 - 1)) as u16;
}

/* Installs the IDT */
extern {
    fn idt_load(x: *IDTPointer);
}

#[no_mangle]
pub unsafe fn idt_install() {
    idt_load(&idtp as *IDTPointer);
    /* Sets the special IDT pointer up, just like in 'gdt.c' */
    idtp.limit = ((super::core::mem::size_of::<IDTEntry>() * 256) - 1) as u16;
    idtp.base = &idt as *[IDTEntry, ..256] as u32;

    /* Add any new ISRs to the IDT here using idt_set_gate */
//    idt_set_gate();
    //asm!("lidt (%0)" : :"p" ())

    /* Points the processor's internal register to the new IDT */
}
		
