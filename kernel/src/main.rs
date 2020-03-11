#![no_std]
#![no_main]
#![feature(
    const_raw_ptr_deref,
    const_if_match
)]

mod initializer;
mod lang;
mod locking;

#[no_mangle]
unsafe extern "C" fn main() -> ! {
    initializer::init();
    ::core::hint::unreachable_unchecked()
}
