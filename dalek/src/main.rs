#![no_std]
#![no_main]

use checkct::{PrivateRng, PublicRng, RngCore};

#[no_mangle]
pub fn main() {
    use x25519_dalek::{PublicKey, EphemeralSecret};
    let mut public_key_bytes = [0u8; 32];
    PublicRng.fill_bytes(&mut public_key_bytes);

    let public = PublicKey::from(public_key_bytes);
    let ephemeral = EphemeralSecret::random_from_rng(PrivateRng);
    let shared_secret = ephemeral.diffie_hellman(&public);
    core::hint::black_box(shared_secret);
}
