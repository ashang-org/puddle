use core::slice::to_ptr;
use core::vec::Vec;

enum HeaderType {
    PT_NULL = 0,
    PT_LOAD = 1,
    PT_DYNAMIC = 2,
    PT_INTERP = 3,
    PT_NOTE = 4,
    PT_SHLIB = 5,
    PT_PHDR = 6
}

#[packed]
struct ELFIdent {
    ei_mag: [u8, ..4], 
    ei_class: u8,
    ei_data: u8,
    ei_version: u8,
    ei_osabi: u8,
    ei_abiversion: u8,
    ei_pad: [u8, ..7]
}

#[packed]
struct ELFHeader {
    e_ident: ELFIdent,
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: u32,
    e_phoff: u32,
    e_shoff: u32,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16
}

#[packed]
struct ProgramHeader {
    p_type: u32,
    p_offset: u32,
    p_vaddr: u32,
    p_paddr: u32,
    p_filesz: u32,
    p_memsz: u32,
    p_flags: u32
}


pub fn read_program_header<'a>(file: &'a [u8], header: &ELFHeader) -> &'a ProgramHeader {
    unsafe {
        let x : *ProgramHeader = to_ptr(file) as *ProgramHeader;
        //let x : *ProgramHeader = file.as_ptr() as *ProgramHeader;
        return &*x;
    }
}


pub fn read_header<'a>(file: &'a [u8]) -> &'a ELFHeader {
    unsafe {
        let x : *ELFHeader = to_ptr(file) as *ELFHeader;
        //let x : *ELFHeader = file.as_ptr() as *ELFHeader;
        return &*x;
    }
}

pub fn read_segments<'a>(file: &'a [u8], header: &ELFHeader) -> &'a [ProgramHeader] {
    let n_headers = header.e_phnum;
    let segments: Vec<u8> = Vec::with_capacity(n_headers as uint);
    {
        let segments_mut = segments.as_mut_slice();
    }
}

#[test]
fn test_read_elf_from_file() {
    use std::io::File;
    use std::io::{Open, ReadWrite};
    let path = Path::new("build/programs/do-nothing");
    let mut stream = File::open_mode(&path, Open, ReadWrite);
    let bytes = stream.read_to_end();
    let header = read_header(bytes);
    // Check the magic bytes
    assert!(header.e_ident.ei_mag.slice(1,4) == "ELF".as_bytes());
    assert!(header.e_entry == 0x80480b8);
}
