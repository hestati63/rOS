#![no_std]
#![no_main]
#![feature(asm, const_raw_ptr_deref, const_if_match, core_intrinsics)]

#[macro_use]
extern crate arch;
mod dev;
mod initializer;
mod lang;
mod locking;
mod mm;

#[no_mangle]
unsafe extern "C" fn main() -> ! {
    initializer::init();
    ::core::hint::unreachable_unchecked()
}
