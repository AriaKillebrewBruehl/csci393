#![no_std] // don't want any of the standard library things
#![no_main] // we don't want the normal entry point chain
use core::panic::PanicInfo;
// disable name mangling so the Rust compiler
// really ourputs a function with the name _start
// "C" means use the C calling convention
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
// function called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
