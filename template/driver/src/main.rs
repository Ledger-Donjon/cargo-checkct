//----- AUTOGENERATED BY CARGO-CHECKCT: DO NOT MODIFY -----
//
#![no_std]
#![no_main]

mod driver;
mod rng;
use driver::checkct;

#[no_mangle]
#[inline(never)]
fn __checkct() {
    core::hint::black_box(rng::__checkct_public_rand());
    checkct()
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    core::hint::black_box(__checkct());
    panic!()
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}