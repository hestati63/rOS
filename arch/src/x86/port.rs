pub trait PortMappedIO {
    fn read_u8(self) -> u8;
    fn read_u16(self) -> u16;
    fn read_u32(self) -> u32;

    fn write_u8(self, data: u8);
    fn write_u16(self, data: u16);
    fn write_u32(self, data: u32);

    fn read_u32s(self, addr: u32, cnt: u32);
}

impl PortMappedIO for u16 {
    #[inline(always)]
    fn read_u8(self) -> u8 {
        let ret: u8;
        unsafe {
            asm!("inb %dx ,%al" : "={al}" (ret) : "{dx}" (self) :
                                : "volatile");
        }
        ret
    }

    #[inline(always)]
    fn read_u16(self) -> u16 {
        let ret: u16;
        unsafe {
            asm!("inw %dx ,%ax" : "={ax}" (ret)
                                : "{dx}" (self) :
                                : "volatile");
        }
        ret
    }

    #[inline(always)]
    fn read_u32(self) -> u32 {
        let ret: u32;
        unsafe {
            asm!("inl %dx ,%eax" : "={eax}" (ret)
                                 : "{dx}" (self) :
                                 : "volatile");
        }
        ret
    }

    #[inline(always)]
    fn write_u8(self, data: u8) {
        unsafe {
            asm!("outb %al ,%dx" :
                                 : "{al}" (data), "{dx}" (self) :
                                 : "volatile");
        }
    }
    #[inline(always)]
    fn write_u16(self, data: u16) {
        unsafe {
            asm!("outw %ax ,%dx" :
                                 : "{ax}" (data), "{dx}" (self) :
                                 : "volatile");
        }
    }

    #[inline(always)]
    fn write_u32(self, data: u32) {
        unsafe {
            asm!("outl %eax ,%dx" :
                                  : "{eax}" (data), "{dx}" (self) :
                                  : "volatile");
        }
    }

    #[inline(always)]
    fn read_u32s(self, addr: u32, cnt: u32) {
        unsafe {
            asm!("cld\n\t\
                  repnz insl (%dx),%es:(%edi)"
                 : : "{edx}" (self), "{edi}" (addr), "{ecx}" (cnt)
                 : "memory", "cc");
        }
    }
}
