use arch::{PortMappedIO, Virtual};
use core::intrinsics::copy;

const CGA_BUF: u64 = 0xB8000;
const CGA_BASE: u16 = 0x3D4;
const MONO_BUF: u64 = 0xB0000;
const MONO_BASE: u16 = 0x3B4;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
const BUFFER_SIZE: usize = BUFFER_HEIGHT * BUFFER_WIDTH;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorMixin(u8);

impl ColorMixin {
    pub const fn new(fg: Color, bg: Color) -> ColorMixin {
        ColorMixin((bg as u8) << 4 | (fg as u8))
    }
}

impl Default for ColorMixin {
    fn default() -> Self {
        ColorMixin::new(Color::LightGray, Color::Black)
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VGAChar {
    pub ascii: u8,
    pub color: ColorMixin,
}

#[repr(transparent)]
pub struct VGABuffer {
    chars: [VGAChar; BUFFER_SIZE],
}

pub struct CGA {
    buffer: &'static mut VGABuffer,
    pos: usize,
    port: u16,
}

unsafe fn init_cga(kbase: &Virtual) -> Result<(*mut VGABuffer, u16), ()> {
    let cp = (kbase.to_u64() + CGA_BUF) as *mut u16;
    *cp = 0xA55A;
    if *cp != 0xA55A {
        Err(())
    } else {
        Ok((cp as *mut VGABuffer, CGA_BASE))
    }
}

fn init_mono(kbase: &Virtual) -> Result<(*mut VGABuffer, u16), ()> {
    let cp = (kbase.to_u64() + MONO_BUF) as *mut VGABuffer;
    Ok((cp, MONO_BASE))
}

impl CGA {
    pub fn init(kbase: Virtual) -> Self {
        unsafe {
            let (cp, port) = init_cga(&kbase).or_else(|_| init_mono(&kbase)).unwrap();
            port.write_u8(14);
            let mut pos = ((port + 1).read_u8() as usize) << 8;
            port.write_u8(15);
            pos |= (port + 1).read_u8() as usize;
            CGA {
                buffer: &mut *cp,
                pos: pos,
                port: port,
            }
        }
    }

    pub fn putc(&mut self, vc: VGAChar) -> () {
        match vc.ascii {
            // \b
            0x8 => {
                if self.pos > 0 {
                    self.pos -= 1;
                    self.buffer.chars[self.pos] = VGAChar {
                        color: vc.color,
                        ascii: b' ',
                    };
                }
            }
            // \n
            0xa => {
                self.pos += BUFFER_WIDTH - (self.pos % BUFFER_WIDTH);
            }
            // \r
            0xd => {
                self.pos -= self.pos % BUFFER_WIDTH;
            }
            // \t
            0x9 => {
                for _ in 0..5 {
                    CGA::putc(
                        self,
                        VGAChar {
                            color: vc.color,
                            ascii: b'_',
                        },
                    );
                }
            }
            _ => {
                self.buffer.chars[self.pos] = vc;
                self.pos += 1;
            }
        }

        if self.pos >= BUFFER_SIZE {
            let buffer_va = self.buffer as *const _ as u64;
            unsafe {
                copy(
                    (buffer_va + BUFFER_WIDTH as u64) as *const _,
                    self.buffer,
                    BUFFER_SIZE - BUFFER_WIDTH,
                );
            }
            for i in (BUFFER_SIZE - BUFFER_WIDTH)..BUFFER_SIZE {
                self.buffer.chars[i] = VGAChar {
                    color: ColorMixin::new(Color::LightGray, Color::Black),
                    ascii: b' ',
                };
            }
            self.pos -= BUFFER_WIDTH;
        }
        self.port.write_u8(14);
        (self.port + 1).write_u8((self.pos >> 8) as u8);
        self.port.write_u8(15);
        (self.port + 1).write_u8(self.pos as u8);
    }
}
