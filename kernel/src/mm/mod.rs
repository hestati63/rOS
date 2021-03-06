mod multiboot;
mod region;
mod zone;

use arch::Virtual;

extern "C" {
    static _end: u64;
}

static mut NEXT_FREE: Option<Virtual> = None;

/// Allocation for the early boot.
/// This should not be called after mm system initialized.
#[link_section = ".init.text"]
fn early_boot_alloc<T>(n: u64) -> Result<&'static mut T, ()> {
    unsafe {
        let alloc_size = page_up!(n);
        let current = NEXT_FREE.unwrap();
        match Virtual::new(current.to_u64() + alloc_size) {
            Ok(virt) => NEXT_FREE = Some(virt),
            Err(_) => panic!("OOM"),
        }
        current.as_mut::<T>().ok_or(())
    }
}

#[link_section = ".init.text"]
pub fn init(kern_base: Virtual) {
    unsafe {
        match Virtual::new(page_up!(&_end as *const _ as u64)) {
            Ok(virt) => NEXT_FREE = Some(virt),
            Err(_) => panic!("OOM"),
        }
    }

    multiboot::read_mb_info(&kern_base)
        .iter_usable()
        .for_each(zone::foster_zone);
    // XXX: The allocator claim memory from the zone.
}
