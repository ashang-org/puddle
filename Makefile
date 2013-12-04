CC=i386-elf-gcc
LD=i386-elf-ld
RUSTC=rustc
NASM=nasm
QEMU=qemu-system-i386
BUILDDIR=./build
OBJDIR=$(BUILDDIR)/obj


OBJS := $(OBJDIR)/runtime.asm.o $(OBJDIR)/main.o $(OBJDIR)/isr_wrapper.asm.o

all: $(BUILDDIR)/floppy.img

.SUFFIXES:

.SUFFIXES: .o .rs .asm

.PHONY: clean run

$(OBJDIR)/%.asm.o: src/%.asm
	mkdir -p $(OBJDIR)
	$(NASM) -f elf32 -o $@ $<

$(OBJDIR)/main.o: src/main.rs
	mkdir -p $(OBJDIR)
	$(RUSTC) -O --target i386-intel-linux --lib -o $@ -c $<

$(BUILDDIR)/loader.bin: src/loader.asm
	mkdir -p $(BUILDDIR)
	$(NASM) -o $@ -f bin $<

$(BUILDDIR)/main.bin: src/linker.ld $(OBJS)
	mkdir -p $(BUILDDIR)
	$(LD) -o $@ -T $^

$(BUILDDIR)/floppy.img: $(BUILDDIR)/loader.bin $(BUILDDIR)/main.bin
	cat $^ > $@

run: $(BUILDDIR)/floppy.img
	$(QEMU) -fda $<

clean:
	-rm -rf $(BUILDDIR)
