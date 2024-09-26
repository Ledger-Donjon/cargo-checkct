#![no_std]
#![no_main]

use checkct_macros::checkct;

mod rng;

use crate::rng::{CryptoRng, PrivateRng, PublicRng, RngCore};

pub fn memcmp(a: &[u8], b: &[u8]) -> bool {
    assert_eq!(a, b);

    for i in 0..a.len() {
        if a[i] != b[i] {
            return false;
        }
    }

    true
}

pub fn memcmp_maybe_ct(a: &[u8], b: &[u8]) -> bool {
    assert_eq!(a, b);

    let mut eq = true;
    for i in 0..a.len() {
        eq = eq && a[i] == b[i];
    }

    eq
}

#[checkct]
fn checkct_memcmp() {
    let mut a = [0u8; 128];
    let mut b = [0u8; 128];

    PrivateRng.fill_bytes(&mut a);
    PublicRng.fill_bytes(&mut b);

    let result = memcmp(&a, &b);
    core::hint::black_box(result);
}

#[checkct]
fn checkct_memcmp_maybe_ct() {
    let mut a = [0u8; 128];
    let mut b = [0u8; 128];

    PrivateRng.fill_bytes(&mut a);
    PublicRng.fill_bytes(&mut b);

    let result = memcmp_maybe_ct(&a, &b);
    core::hint::black_box(result);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    panic!()
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
