use arch::{PG_SHIFT, Physical};
use crate::locking::SpinLock;
use super::region::Region;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(C)]
pub enum zone_type {
    /// For the device that only support 0 ~ 16MB address space.
    ZoneDMA = 0,
    /// For the device that only support 0 ~ 4G address space.
    ZoneDMA32 = 1,
    /// Remainder
    ZoneHighMem = 2,
}

fn zone_type(addr: u64) -> zone_type {
    // < 16MB
    if addr < 0x1000000 {
        zone_type::ZoneDMA
    } else if addr < 0x100000000 {
        zone_type::ZoneDMA32
    } else {
        zone_type::ZoneHighMem
   }
}

fn end_of_zone(zt: zone_type) -> u64 {
    match zt {
        zone_type::ZoneDMA => 0x1000000 - 1,
        zone_type::ZoneDMA32 => 0x100000000 - 1,
        zone_type::ZoneHighMem => 0xffffffffffffffff,
    }
}

fn order(size: u64) -> usize {
    match page_up!(size) >> PG_SHIFT {
       v if v <= 0x1 => 0,
       v if v <= 0x2 => 1,
       v if v <= 0x4 => 2,
       v if v <= 0x8 => 3,
       v if v <= 0x10 => 4,
       v if v <= 0x20 => 5,
       v if v <= 0x40 => 6,
       v if v <= 0x80 => 7,
       v if v <= 0x100 => 8,
       v if v <= 0x200 => 9,
       _ => 10,
    }
}

static ZONES: [SpinLock<Zone>; 3] = [
    SpinLock::new(Zone::init()),
    SpinLock::new(Zone::init()),
    SpinLock::new(Zone::init()),
];

#[derive(Debug)]
struct FreeArea {
    addr: Physical,
    next: Option<&'static FreeArea>
}

#[derive(Debug)]
struct Zone {
    // zone_start_pfn == zone_start_paddr >> PAGE_SHIFT
    start_pfn       : u64,
    // total pages spanned by the zone.
    total_pages     : u64,
    // available pages in the zone. (freed)
    available_pages : u64,
    // free areas of diffrent size.
    free_area       : [Option<&'static FreeArea>; 10],
}

impl Zone {
    const fn init() -> Self {
        Zone {
            start_pfn       : 0,
            total_pages     : 0,
            available_pages : 0,
            free_area       : [None; 10]
        }
    }

    const fn is_initialized(&self) -> bool {
        self.total_pages != 0
    }

    pub fn push_region(&mut self, region: Region) {
        crate::println!("Push Zone: {}", region);
        if self.is_initialized() {
            self.start_pfn = pfn!(region.addr);
        }
        self.total_pages += region.len >> PG_SHIFT;
        // FIXME: all the pages in the zone's are not available
        self.available_pages += region.len >> PG_SHIFT;
        // XXX: slab initialized here first.
        // TODO: Make page object and push them into the free_area
    }
}

pub fn foster_zone(region: Region) {
    let start = region.addr;
    let end = region.next_addr() - 1;

    if zone_type(start) != zone_type(end) {
        let border = end_of_zone(zone_type(start));
        // split the region
        foster_zone(Region {
            addr  : start,
            len   : border - start,
            mtype : region.mtype
        });
        foster_zone(Region {
            addr  : border + 1,
            len   : end - border - 1,
            mtype : region.mtype
        });
    } else {
        ZONES[zone_type(end) as usize]
            .borrow()
            .push_region(region);
    }
}
