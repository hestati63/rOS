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

    pub unsafe fn as_mut<T>(&self) -> Option<&'static mut T> {
        (self.to_u64() as *mut T).as_mut()
    }

    pub unsafe fn as_ref<T>(&self) -> Option<&'static T> {
        (self.to_u64() as *mut T).as_ref()
    }
}

pub const PG_SHIFT: u64 = 12;
pub const PAGE_SIZE: u64 = 1 << PG_SHIFT;
pub const PAGE_MASK: u64 = PAGE_SIZE - 1;

#[macro_export]
macro_rules! page_up {
    ($arg: expr) => {
        ($arg + $crate::PAGE_MASK) & !$crate::PAGE_MASK
    };
}

#[macro_export]
macro_rules! page_down {
    ($arg: expr) => {
        $arg & !$crate::PAGE_MASK
    };
}

#[macro_export]
macro_rules! pfn {
    ($arg: expr) => {
        $arg >> $crate::PG_SHIFT
    };
}
