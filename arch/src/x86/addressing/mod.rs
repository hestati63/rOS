#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(transparent)]
pub struct Physical(u64);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(transparent)]
pub struct Virtual(u64);

impl Physical {
    pub const fn new(addr: u64) -> Self {
        Physical(addr)
    }

    pub const fn to_u64(&self) -> u64 {
        self.0
    }
}

impl Virtual {
    // bits 48..64 should be zero or sign-extended.
    #[inline(always)]
    pub fn new(addr: u64) -> Result<Self, ()> {
        match addr & 0xffff_8000_0000_0000 {
            0 | 0xffff_8000_0000_0000 => Ok(Virtual(addr)),
            _ => Err(()),
        }
    }

    pub const fn to_u64(&self) -> u64 {
        self.0
    }
}
