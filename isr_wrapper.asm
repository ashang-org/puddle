use32
global  _interrupt_handler_kbd_wrapper
global  int_handler
global test1                    
global  run_interrupt
align   4

extern _interrupt_handler_kbd

_interrupt_handler_kbd_wrapper: 
    pushad
    call    _interrupt_handler_kbd 
    popad
    iret


run_interrupt:
   int 1
   int 1
   int 1
   int 1
   int 1
   ret

int_handler:
   pushad
   mov ax, 0x08
   mov gs, ax
   mov dword [gs:0xB8000],': ) '
   call  _interrupt_handler_kbd 
   popad
   iret

idt:
   resd 50*2

idtr:
   dw (50*8)-1
   dd idt

test1:
   lidt [idtr]

   mov eax,int_handler
   mov [idt+49*8],ax
   mov word [idt+49*8+2],0x08
   mov word [idt+49*8+4],0x8E00
   shr eax,16
   mov [idt+49*8+6],ax

   mov eax,int_handler
   mov [idt+1*8],ax
   mov word [idt+1*8+2],0x08
   mov word [idt+1*8+4],0x8E00
   shr eax,16
   mov [idt+1*8+6],ax

   ret

