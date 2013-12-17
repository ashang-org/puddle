use32
global  _interrupt_handler_kbd_wrapper
global  int_handler
global test1                    
global  run_interrupt
align   4

extern _interrupt_handler_kbd
extern idt

run_interrupt:
   int 33
   ret

int_handler:
   pushad
   mov ax, 0x08
   mov gs, ax
   call  _interrupt_handler_kbd 
   popad
   iret

test1:
   lidt [idtr]
   ret

idtr:
   dw (50*8)-1
   dd idt

