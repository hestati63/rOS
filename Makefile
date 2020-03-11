builddir := build
profile ?= debug
kern := $(builddir)/kern.bin
boot := $(builddir)/boot.bin
img  := $(builddir)/bootimg.bin
disk_size = $(shell du --apparent-size --block-size=1 $(boot) | cut -f1)
disk_sectors = $(shell echo $(( $(disk_size) + 512)))
CPUS ?= 4

ifeq ($(profile), release)
APPEND := --release
else
APPEND :=
endif


all: prepare image

prepare:
	mkdir -p $(builddir)

$(boot): bootloader
$(kern): kernel

bootloader:
	RUST_TARGET_PATH=$(shell pwd)/scripts \
		xargo build --target bootloader $(APPEND) -p bootloader
	objdump -d target/bootloader/$(profile)/bootloader > $(builddir)/bootloader.asm
	rust-objcopy -I elf64-x86-64 -O binary --binary-architecture=i386:x86-64 \
		target/bootloader/$(profile)/bootloader $(boot)

kernel:
	RUST_TARGET_PATH=$(shell pwd)/scripts \
		xargo build --target kernel $(APPEND) -p kernel
	objdump -d target/kernel/$(profile)/kernel > $(builddir)/kernel.asm
	rust-objcopy -I elf64-x86-64 -O binary --binary-architecture=i386:x86-64 \
		target/kernel/$(profile)/kernel $(kern)


image: $(boot) $(kern)
	dd if=/dev/zero of=$(img)~ bs=512 count=20000 2>/dev/null
	dd if=$(boot) of=$(img)~ conv=notrunc 2>/dev/null
	dd if=$(kern) of=$(img)~ \
		seek=$$(($$(($(disk_size)+511)) / 512)) conv=notrunc
	mv $(img)~ $(img)

run: image
	@qemu-system-x86_64 -drive format=raw,file=$(img) -cpu qemu64 \
		-m 256 -nographic -no-reboot \
		-smp $(CPUS) \
		-net user -net nic,model=e1000 \
		-serial mon:stdio

clean:
	@rm -rf $(builddir) target

.PHONY: all prepare clean bootloader kernel
