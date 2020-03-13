pub trait PerCPUSafe {}
impl PerCPUSafe for i8 {}
impl PerCPUSafe for i16 {}
impl PerCPUSafe for i32 {}
impl PerCPUSafe for i64 {}
impl PerCPUSafe for u8 {}
impl PerCPUSafe for u16 {}
impl PerCPUSafe for u32 {}
impl PerCPUSafe for u64 {}
impl PerCPUSafe for usize {}
impl<T> PerCPUSafe for *const T {}
impl<T> PerCPUSafe for *mut T {}

/// PerCPU object. We use gs based addressing for the x86_64.
pub trait PerCPU {
    type T: PerCPUSafe;

    fn get() -> Self::T;
    fn set(v: Self::T);
    unsafe fn offset() -> u64;
}

extern "C" {
    pub static __per_cpu_start: u64;
}


pub union PointerHack<T: 'static> {
    reference: &'static T,
    pub addr: u64
}

impl<T: 'static> PointerHack<T> {
    pub const fn __new(d: &'static T) -> Self {
        PointerHack { reference: d }
    }
}

// Proxy for generating
#[macro_export]
macro_rules! per_cpu {
    (static mut $N:ident : $T:ty = $e:expr;) => {
        per_cpu!($N, $T, $e);
        #[allow(non_upper_case_globals)]
        const $N: $N::$N = $N::$N {};
    };

    (pub static mut $N:ident : $T:ty = $e:expr;) => {
        per_cpu!($N, $T, $e);
        #[allow(non_upper_case_globals)]
        pub const $N: $N::$N = $N::$N {};
    };

    ($N: ident, $T: ty, $e: expr) => {
        mod $N {
            mod sealed {
                #[used]
                #[link_section = ".percpu.data"]
                #[allow(non_upper_case_globals)]
                pub static mut $N: $T = $e; // Object for calculating offset
            }

            #[allow(non_camel_case_types)]
            pub struct $N {}

            impl $crate::PerCPU for $N {
                type T = $T;

                #[inline(always)]
                unsafe fn offset() -> u64 {
                   &sealed::$N as *const $T as u64
                       - &$crate::__per_cpu_start as *const $T as u64
                }

                #[inline(always)]
                fn get() -> Self::T {
                    unsafe {
                        let ret: Self::T;
                        asm!("mov %gs:($1), $0"
                            : "=r"(ret)
                            : "r"(Self::offset())
                            :: "volatile");
                        ret
                    }
                }

                #[inline(always)]
                fn set(v: Self::T) {
                    unsafe {
                        asm!("mov $0, %gs:($1)"
                            :
                            : "r" (v), "r"(Self::offset())
                            :: "volatile");
                    }
                }

            }
        }
    };
}
