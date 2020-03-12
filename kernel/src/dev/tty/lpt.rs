use arch::PortMappedIO;

pub struct LPT;

impl LPT {
    const BASE: u16 = 0x378;

    pub fn putc(vc: u8) {
        for _ in 0..12800 {
            if (Self::BASE + 1).read_u8() & 0x80 != 0 {
                break;
            } else {
                // delay
                0x84.read_u8();
                0x84.read_u8();
                0x84.read_u8();
                0x84.read_u8();
            }
        }
        (Self::BASE + 0).write_u8(vc);
        (Self::BASE + 2).write_u8(0xf);
        (Self::BASE + 2).write_u8(0x8);
    }
}
