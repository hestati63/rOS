use arch::Virtual;

extern "C" {
    static _edata: u64;
    static _end: u64;
}

// To ensure that all static/global variables are zero,
// the kernel zeros out the bss section first.
#[link_section = ".init.text"]
fn __cleanup_bss() {
    unsafe {
        let edata = &_edata as *const _ as u64;
        let end = &_end as *const _ as u64;
        core::intrinsics::write_bytes(
            edata as *mut u8,
            0,
            (end - edata) as usize,
        );
    }
}

#[link_section = ".init.text"]
pub fn init() {
    __cleanup_bss();
    crate::dev::tty::init(Virtual::new(0x8004000000).unwrap());
    crate::mm::init(Virtual::new(0x8004000000).unwrap());
    unimplemented!();
    // TODO: mm
    // TODO: mp
    // TODO: trap
    // TODO: sched
}
