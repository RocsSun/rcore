use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "panic at: {} {} {}",
            location.file(),
            location.line(),
            info.message(),
        )
    } else {
        println!("panic at: {}", info.message())
    };
    loop {}
}
