#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    crate::println_unlocked!("{}", info);
    loop {}
}
