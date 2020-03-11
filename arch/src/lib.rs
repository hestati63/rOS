#![allow(unused_imports)]
#![feature(asm)]
#![cfg_attr(not(test), no_std)]

#[cfg(target_arch = "x86_64")]
mod x86;
#[cfg(target_arch = "x86_64")]
pub use x86::*;
