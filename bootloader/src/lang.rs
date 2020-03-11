use arch::PortMappedIO;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    0xf4.write_u32(0x10);
    loop {}
}
