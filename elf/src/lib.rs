#![cfg_attr(not(test), no_std)]
#![cfg_attr(feature = "deny-warnings", deny(warnings))]

pub mod fmt;

use core::slice;

use fmt::*;

#[derive(Debug)]
pub struct ELF {
    hdr: ELFHeader64,
    inp: *const u8,
}

#[derive(Debug)]
pub struct PhdrIter<'a> {
    pub phdrs: &'a [ProgHeader64],
    pub cursor: u16,
    pub size: u16,
}

impl<'a> Iterator for PhdrIter<'a> {
    type Item = ProgHeader64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.size > self.cursor {
            let result = self.phdrs[self.cursor as usize];
            self.cursor += 1;
            return Some(result);
        }
        None
    }
}

impl<'a> ELF {
    pub fn new(inp: *const u8) -> Result<Self, ()> {
        unsafe {
            match (inp as *mut ELFHeader64).as_ref() {
                Some(hdr) if hdr.ei_magic == 0x464C457F => Ok(ELF {
                    hdr: *hdr,
                    inp: inp,
                }),
                _ => Err(()),
            }
        }
    }

    pub const fn entry(&self) -> u64 {
        self.hdr.e_entry
    }

    pub unsafe fn phdrs(&self) -> PhdrIter<'a> {
        PhdrIter {
            size: self.hdr.e_phnum,
            phdrs: slice::from_raw_parts(
                (self.inp as u64 + self.hdr.e_phoff) as *mut ProgHeader64,
                self.hdr.e_phnum as usize,
            ),
            cursor: 0,
        }
    }
}
