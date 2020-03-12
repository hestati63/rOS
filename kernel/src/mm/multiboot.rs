use super::region::{MemoryRegion, Region, RegionType};
use arch::Virtual;
use core::mem::size_of;
use core::slice;

#[repr(C)]
struct MBInfo {
    flags: u32,
    mem_low: u32,
    mem_hi: u32,
    boot_dev: u32,
    cmdline: u32,
    mods_count: u32,
    mods_addr: u32,
    _1: u32,
    _2: u32,
    _3: u32,
    _4: u32,
    mmap_len: u32,
    mmap_addr: u32,
}

#[repr(C)]
struct E820Entry {
    size: u32,
    mem_lo: u32,
    mem_hi: u32,
    len_lo: u32,
    len_hi: u32,
    type_: u32,
}

impl From<&E820Entry> for Region {
    #[link_section = ".init.text"]
    fn from(mm: &E820Entry) -> Region {
        Region {
            addr: (mm.mem_lo as u64) | ((mm.mem_hi as u64) << 32),
            len: (mm.len_lo as u64) | ((mm.len_hi as u64) << 32),
            mtype: match mm.type_ {
                1 => RegionType::Usable,
                3 => RegionType::AcpiReclaimable,
                4 => RegionType::AcpiNvs,
                5 => RegionType::BadMemory,
                _ => RegionType::Reserved,
            },
        }
    }
}

#[link_section = ".init.text"]
unsafe fn init_from_mbinfo(bootinfo: &MBInfo) -> MemoryRegion {
    let mut regions = MemoryRegion::new();
    let entry_counts = (bootinfo.mmap_len as usize) / size_of::<E820Entry>();
    slice::from_raw_parts(bootinfo.mmap_addr as *mut E820Entry, entry_counts)
        .iter()
        .for_each(|entry| regions.add(Region::from(entry)));
    regions.show_info();
    regions
}

#[link_section = ".init.text"]
pub fn read_mb_info(kern_base: &Virtual) -> MemoryRegion {
    unsafe {
        init_from_mbinfo(
            ((kern_base.to_u64() + 0x7000) as *mut MBInfo)
                .as_mut()
                .unwrap(),
        )
    }
}
