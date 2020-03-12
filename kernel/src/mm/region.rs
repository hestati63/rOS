use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Region {
    pub addr: u64,
    pub len: u64,
    pub mtype: RegionType,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum RegionType {
    /// Unused memory, can be freely used by the kernel.
    Usable,
    /// Memory that is already in use.
    InUse,
    /// Memory reserved by the hardware. Not usable.
    Reserved,
    /// ACPI reclaimable memory
    AcpiReclaimable,
    /// ACPI NVS memory
    AcpiNvs,
    /// Area containing bad memory
    BadMemory,
    /// Memory used for loading the kernel.
    Kernel,
    /// Memory used for the kernel stack.
    KernelStack,
    /// Memory used for creating page tables.
    PageTable,
    /// Memory used by the bootloader.
    Bootloader,
    /// Frame at address zero.
    FrameZero,
    /// An empty region with size 0
    Empty,
    /// Memory used for storing the boot information.
    BootInfo,
    /// Memory used for storing the supplied package
    Package,
    /// Additional variant to ensure that we can add more variants in the future without
    /// breaking backwards compatibility.
    #[doc(hidden)]
    NonExhaustive,
}

impl Region {
    const fn new() -> Self {
        Region {
            addr: 0,
            len: 0,
            mtype: RegionType::Usable,
        }
    }

    const fn next_addr(&self) -> u64 {
        self.addr + self.len
    }
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} (0x{:X} ~ 0x{:X})",
            self.mtype,
            self.addr,
            self.next_addr() - 1,
        )
    }
}

#[repr(C)]
pub struct MemoryRegion {
    regions: [Region; 64],
    index: usize,
    total_size: u64,
    last_page: u64,
}

impl MemoryRegion {
    pub const fn new() -> Self {
        MemoryRegion {
            regions: [Region::new(); 64],
            index: 0,
            total_size: 0,
            last_page: 0,
        }
    }

    fn update_meta(&mut self, d: &Region) {
        self.total_size += d.len;
        if d.next_addr() > self.last_page {
            self.last_page = d.next_addr();
        }
    }

    fn try_merge_at(&mut self, i: usize, d: Region) -> Result<(), ()> {
        if self.regions[i].next_addr() == d.addr
            && self.regions[i].mtype == d.mtype
        {
            self.regions[i] = Region {
                addr: self.regions[i].addr,
                len: self.regions[i].len + d.len,
                mtype: d.mtype,
            };
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn show_info(&self) {
        crate::println!("Memory Available: {}MB", self.total_size / 1024 / 1024);
        crate::println!("Total {} pages available.", self.total_size / arch::PAGE_SIZE);

    }

    pub fn add(&mut self, d: Region) {
        self.update_meta(&d);
        for i in 0..self.index {
            if self.regions[i].addr > d.addr {
                // Now we find index to insert
                if self.try_merge_at(i, d).is_err() {
                    for j in 0..(self.index - i) {
                        self.regions[self.index - j] =
                            self.regions[self.index - j - 1];
                    }
                    self.regions[i] = d;
                    self.index += 1;
                }
                return;
            }
        }

        // forward merge
        if self.index == 0 || self.try_merge_at(self.index - 1, d).is_err() {
            self.regions[self.index] = d;
            self.index += 1;
        }
    }

    pub fn iter_usable(&self) -> RegionIter {
        RegionIter {
            cursor: 0,
            memory_region: self,
        }
    }
}

pub struct RegionIter<'a> {
    cursor: usize,
    memory_region: &'a MemoryRegion,
}

impl<'a> Iterator for RegionIter<'a> {
    type Item = Region;

    fn next(&mut self) -> Option<Self::Item> {
        while self.memory_region.index > self.cursor {
            let result = self.memory_region.regions[self.cursor];
            self.cursor += 1;
            match result.mtype {
                RegionType::Usable | RegionType::AcpiReclaimable => {
                    return Some(result);
                }
                _ => {}
            }
        }
        None
    }
}

impl fmt::Display for MemoryRegion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[MemoryRegions ({})]\n", self.index)?;
        for i in 0..self.index {
            write!(f, "\t{}\n", self.regions[i])?;
        }
        write!(f, "==============")
    }
}
