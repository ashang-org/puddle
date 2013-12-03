use32
global  _interrupt_handler_kbd_wrapper
global  int_handler
align   4

extern _interrupt_handler_kbd

_interrupt_handler_kbd_wrapper: 
    pushad
    call    _interrupt_handler_kbd 
    popad
    iret

int_handler:
   mov ax, 0x08
   mov gs, ax
   mov dword [gs:0xB8000],') : '
   hlt
