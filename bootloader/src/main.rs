#![feature(asm)]
#![feature(global_asm)]
#![no_std]
#![no_main]

mod disk;
mod lang;

use disk::Disk;
use elf::ELF;

global_asm!(include_str!("boot.s"));
global_asm!(include_str!("bootstrap.s"));

extern "C" {
    static boot_start: u64;
    static boot_end: u64;
}

const KERN_ELF_BASE: u64 = 0x20000;

unsafe fn readseg(pa: u32, count: u32, offset: u32) {
    let mut addr: u32 = pa & !(Disk::BLOCK_SIZE - 1);
    // FIXME: add round_down and change to it.
    let mut sect: u32 = offset / Disk::BLOCK_SIZE;
    while addr < pa + count {
        Disk::read_sector(addr, sect);
        addr += Disk::BLOCK_SIZE;
        sect += 1;
    }
}

#[no_mangle]
unsafe extern "C" fn boot_main() -> ! {
    let bootloader_start = &boot_start as *const _ as u64;
    let bootloader_end = &boot_end as *const _ as u64;
    let kern_start = (bootloader_end - bootloader_start) as u32;

    readseg(KERN_ELF_BASE as u32, 0x1000, kern_start);

    match ELF::new(KERN_ELF_BASE as *const u8) {
        Ok(elf) => {
            // Currently, bootloader assumes phdr lies on the first page.
            // This should be fixed later.
            for phdr in elf.phdrs() {
                readseg(
                    phdr.p_paddr as u32,
                    phdr.p_filesz as u32,
                    phdr.p_offset as u32 + kern_start,
                )
            }

            // Now, the kernel loaded into the memory.
            // The only remaining thing is to jump into the kernel entry
            asm!("mov $$0x200000, %rsp\n\t
                  jmpq *%rax" : :
                 "{rax}"(elf.entry())
                 : : "volatile");
            ::core::hint::unreachable_unchecked()
        }
        Err(_) => panic!(),
    }
}
