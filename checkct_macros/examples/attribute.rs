use checkct_macros::checkct;

fn memcmp(a: &[u8], b: &[u8]) -> bool {
    assert_eq!(a, b);

    for i in 0..a.len() {
        if a[i] != b[i] {
            return false;
        }
    }

    true
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

#[checkct(descriptor_link_section = ".rodata")]
fn checkct_memcmp2() {
    let mut a = [0u8; 128];
    let mut b = [0u8; 128];

    PrivateRng.fill_bytes(&mut a);
    PublicRng.fill_bytes(&mut b);

    let result = memcmp(&a, &b);
    core::hint::black_box(result);
}

fn main() {}

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
        (__checkct_private_rand() as u32) << 24
            | (__checkct_private_rand() as u32) << 16
            | (__checkct_private_rand() as u32) << 8
            | (__checkct_private_rand() as u32)
    }

    fn next_u64(&mut self) -> u64 {
        (self.next_u32() as u64) << 32 | (self.next_u32() as u64)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        for d in dest {
            *d = __checkct_private_rand();
        }
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
        (__checkct_public_rand() as u32) << 24
            | (__checkct_public_rand() as u32) << 16
            | (__checkct_public_rand() as u32) << 8
            | (__checkct_public_rand() as u32)
    }

    fn next_u64(&mut self) -> u64 {
        (self.next_u32() as u64) << 32 | (self.next_u32() as u64)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        for d in dest {
            *d = __checkct_public_rand();
        }
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

impl CryptoRng for PublicRng {}
