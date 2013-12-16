use core::mem::size_of;

static mut base: uint = 0x200000;

// Naive malloc
#[no_mangle]
pub extern "C" fn malloc(len: uint) -> *mut u8 {
    unsafe {
        let ret: uint = base;
        base += len + size_of::<uint>();

        // Align next allocation to 4-byte boundary.
        if(base % 4 != 0) {
            base += 4 - (base % 4);
        }

        *(base as *mut uint) = len;

        return (ret + size_of::<uint>()) as *mut u8;
    }
}

// Even more naive free().
#[no_mangle]
pub extern "C" fn free(ptr: *mut u8) {
    
}


pub fn main() {}
