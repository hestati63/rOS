builddir := build
kern := $(builddir)/kern.bin
boot := $(builddir)/boot.bin
img  := $(builddir)/bootimg.bin
target ?= bootloader
disk_size = $(shell du --apparent-size --block-size=1 $(bootloader) | cut -f1)
disk_sectors = $(shell echo $(( $(disk_size) + 512)))

CPUS ?= 4

all: prepare $(boot)

prepare:
	mkdir -p $(builddir)

$(boot): bootloader
$(kern): kernel

bootloader:
	RUST_TARGET_PATH=$(shell pwd) xargo build --target $(target)
	objdump -D target/bootloader/debug/bootloader > $(builddir)/bootloader.asm
	rust-objcopy -I elf64-x86-64 -O binary --binary-architecture=i386:x86-64 \
		target/bootloader/debug/bootloader build/bootloader.bin

kernel:

image: $(boot) $(kern)
	dd if=/dev/zero of=$(img)~ bs=512 count=20000 2>/dev/null
	dd if=$(bootloader) of=$(img)~ conv=notrunc 2>/dev/null
	dd if=$(kernel) of=$(img)~ \
		seek=$$(($$(($(disk_size)+511)) / 512)) conv=notrunc
	mv $(img)~ $(img)

run: image
	@qemu-system-x86_64 -drive format=raw,file=$(img) -cpu qemu64 \
		-m 256 -nographic -no-reboot \
		-smp $(CPUS) \
		-net user -net nic,model=e1000 \
		-serial mon:stdio

clean:
	@rm -rf $(builddir)

.PHONY: all prepare clean bootloader
