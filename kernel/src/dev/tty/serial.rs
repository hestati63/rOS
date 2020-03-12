use arch::PortMappedIO;

// Opaque object for serial
pub struct Serial;

impl Serial {
    const BASE: u16 = 0x3F8;
    const RX: u16 = 0; // In: Receive buffer (DLAB=0)
    const TX: u16 = 0; // Out: Transmit buffer (DLAB=0)
    const DLL: u16 = 0; // Out: Divisor Latch Low (DLAB=1)
    const DLM: u16 = 1; // Out: Divisor Latch High (DLAB=1)
    const IER: u16 = 1; // Out: Interrupt Enable Register
    const IER_RDI: u8 = 1; //   Enable receiver data interrupt
    const IIR: u16 = 2; // In: Interrupt ID Register
    const FCR: u16 = 2; // Out: FIFO Control Register
    const LCR: u16 = 3; // Out: Line Control Register
    const LCR_DLAB: u8 = 0x80; //   Divisor latch access bit
    const LCR_WLEN8: u8 = 0x03; //   Wordlength: 8 bits
    const MCR: u16 = 4; // Out: Modem Control Register
    const MCR_RTS: u8 = 0x02; // RTS complement
    const MCR_DTR: u8 = 0x01; // DTR complement
    const MCR_OUT2: u8 = 0x08; // Out2 complement
    const LSR: u16 = 5; // In: Line Status Register
    const LSR_DATA: u8 = 0x01; //   Data available
    const LSR_TXRDY: u8 = 0x20; //   Transmit buffer avail
    const LSR_TSRE: u8 = 0x40; //   Transmitter off

    pub fn init() -> Result<Self, ()> {
        // turn off FIFO
        (Self::BASE + Self::FCR).write_u8(0);
        // set speed
        (Self::BASE + Self::LCR).write_u8(Self::LCR_DLAB);
        (Self::BASE + Self::DLL).write_u8((115200 / 9600) as u8);
        (Self::BASE + Self::DLM).write_u8(0);
        // 8 data bits, 1 stop bit, parity off; turn off DLAB latch
        (Self::BASE + Self::LCR).write_u8(Self::LCR_WLEN8 & !Self::LCR_DLAB);
        // No modem controls
        (Self::BASE + Self::MCR).write_u8(0);
        // Enable rcv interrupts
        (Self::BASE + Self::IER).write_u8(Self::IER_RDI);

        // Clear any preexisting overrun indications and interrupts
        // Serial port doesn't exist if COM_LSR returns 0xFF
        let ser = if (Self::BASE + Self::LSR).read_u8() != 0xFF {
            Ok(Serial)
        } else {
            Err(())
        };
        (Self::BASE + Self::IIR).read_u8();
        (Self::BASE + Self::RX).read_u8();
        ser
    }

    pub fn putc(c: u8) {
        for _ in 0..12800 {
            if (Self::BASE + Self::LSR).read_u8() & Self::LSR_TXRDY != 0 {
                break;
            }
            // delay
            0x84.read_u8();
            0x84.read_u8();
            0x84.read_u8();
            0x84.read_u8();
        }
        (Self::BASE + Self::TX).write_u8(c);
    }
}
