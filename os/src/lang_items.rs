use core::panic::PanicInfo;

use crate::sbi::shutdown;

#[panic_handler]
pub fn panics(info: &PanicInfo) -> ! {
    if let Some(locat) = info.location() {
        println!(
            "panicked at: {} {} {}",
            locat.file(),
            locat.line(),
            info.message(),
        );
    } else {
        println!("panicked at: {}", info.message());
    }
    shutdown(true)
}
