use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut, Drop};
use core::sync::atomic::{spin_loop_hint, AtomicBool, Ordering};

#[derive(Debug)]
pub struct SpinLock<T: ?Sized> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

pub struct SpinLocked<'a, T: ?Sized + 'a> {
    lock: &'a AtomicBool,
    data: &'a mut T,
}

impl<'a, T> core::fmt::Display for SpinLocked<'a, T>
where
    T: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl<'a, T> core::fmt::Debug for SpinLocked<'a, T>
where
    T: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

impl<T> SpinLock<T> {
    pub const fn new(data: T) -> SpinLock<T> {
        SpinLock {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    fn acquire(&self) {
        while self.locked.compare_and_swap(false, true, Ordering::Acquire) {
            while self.locked.load(Ordering::Relaxed) {
                spin_loop_hint();
            }
        }
    }

    pub fn borrow(&self) -> SpinLocked<T> {
        self.acquire();
        SpinLocked {
            lock: &self.locked,
            data: unsafe { &mut *self.data.get() },
        }
    }

    pub fn try_borrow(&self) -> Option<SpinLocked<T>> {
        if !self.locked.compare_and_swap(false, true, Ordering::Acquire) {
            Some(SpinLocked {
                lock: &self.locked,
                data: unsafe { &mut *self.data.get() },
            })
        } else {
            None
        }
    }
}

unsafe impl<T: ?Sized + Send> Sync for SpinLock<T> {}
unsafe impl<T: ?Sized + Send> Send for SpinLock<T> {}

impl<'a, T: ?Sized> Deref for SpinLocked<'a, T> {
    type Target = T;
    fn deref<'b>(&'b self) -> &'b T {
        &*self.data
    }
}

impl<'a, T: ?Sized> DerefMut for SpinLocked<'a, T> {
    fn deref_mut<'b>(&'b mut self) -> &'b mut T {
        &mut *self.data
    }
}

impl<'a, T: ?Sized> Drop for SpinLocked<'a, T> {
    fn drop(&mut self) {
        assert_eq!(self.lock.load(Ordering::Relaxed), true);
        self.lock.store(false, Ordering::Release);
    }
}
