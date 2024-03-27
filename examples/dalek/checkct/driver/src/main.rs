#![no_std]
#![no_main]
        
#[no_mangle]
#[inline(never)]
fn __checkct() {
    // USER CODE GOES HERE
    use dalek::{PublicKey, EphemeralSecret, SharedSecret};
    let mut public_key_bytes = [0u8; 32];
    PublicRng.fill_bytes(&mut public_key_bytes);

    let public = PublicKey::from(public_key_bytes);
    let ephemeral = EphemeralSecret::random_from_rng(PrivateRng);
    let shared_secret = ephemeral.diffie_hellman(&public);
    core::hint::black_box(shared_secret);
}


//----- AUTOGENERATED BY CHECKCT: DO NOT MODIFY -----
//
#[no_mangle]
#[inline(never)]
fn __checkct_private_rand() -> u8 {
    unsafe {
        core::ptr::read_volatile(0xcafe as *const u8)
    }
}

#[no_mangle]
#[inline(never)]
fn __checkct_public_rand() -> u8 {
    unsafe {
        core::ptr::read_volatile(0xf00d as *const u8)
    }
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

use rand_core::{CryptoRng, RngCore};

struct PrivateRng;

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

struct PublicRng;

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