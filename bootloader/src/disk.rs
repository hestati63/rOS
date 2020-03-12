use arch::PortMappedIO;

// opaque object for disk
pub struct Disk;

impl Disk {
    pub const BLOCK_SIZE: u32 = 512;

    #[inline(always)]
    unsafe fn wait() {
        while 0x1f7.read_u8() & 0xC0 != 0x40 {}
    }

    #[inline(never)]
    pub unsafe fn read_sector(pa: u32, sect: u32) {
        Self::wait();
        0x1F2.write_u8(1);
        0x1F3.write_u8((sect >> 0) as u8);
        0x1F4.write_u8((sect >> 8) as u8);
        0x1F5.write_u8((sect >> 16) as u8);
        0x1F6.write_u8(((sect >> 24) | 0xE0) as u8);
        0x1F7.write_u8(0x20);
        Self::wait();
        0x1F0.read_u32s(pa, Self::BLOCK_SIZE / 4);
    }
}
