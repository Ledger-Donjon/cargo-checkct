#![no_std]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// Functions to be tested in user code should be called
// main, with a #[no_mangle] on top
extern "Rust" {
    fn main();
}

#[no_mangle]
#[inline(never)]
pub fn __checkct() {
    core::hint::black_box(__checkct_public_rand());
    unsafe { main() }
    __check_ct_exit();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    core::hint::black_box(__checkct());
    panic!()
}

#[no_mangle]
#[inline(never)]
fn __check_ct_exit() {
    core::hint::black_box(0);
}


#[no_mangle]
#[inline(never)]
pub fn __checkct_private_rand() -> u8 {
    unsafe { core::ptr::read_volatile(0xcafe as *const u8) }
}

#[no_mangle]
#[inline(never)]
pub fn __checkct_public_rand() -> u8 {
    unsafe { core::ptr::read_volatile(0xf00d as *const u8) }
}

pub use rand_core::{CryptoRng, RngCore};

pub struct PrivateRng;

impl PrivateRng {
    pub const fn new() -> Self {
        Self
    }
}

impl RngCore for PrivateRng {
    fn next_u32(&mut self) -> u32 {
        u32::from_be_bytes(core::array::from_fn(|_| __checkct_private_rand()))
    }

    fn next_u64(&mut self) -> u64 {
        u64::from_be_bytes(core::array::from_fn(|_| __checkct_private_rand()))
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        dest.fill_with(__checkct_private_rand);
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

impl CryptoRng for PrivateRng {}

pub struct PublicRng;

impl PublicRng {
    pub const fn new() -> Self {
        Self
    }
}

impl RngCore for PublicRng {
    fn next_u32(&mut self) -> u32 {
        u32::from_be_bytes(core::array::from_fn(|_| __checkct_public_rand()))
    }

    fn next_u64(&mut self) -> u64 {
        u64::from_be_bytes(core::array::from_fn(|_| __checkct_public_rand()))
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        dest.fill_with(__checkct_public_rand);
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

impl CryptoRng for PublicRng {}
