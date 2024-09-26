use crate::rng::{CryptoRng, PrivateRng, PublicRng, RngCore};
use checkct_macros::checkct;

#[checkct]
pub fn checkct() {
    // USER CODE GOES HERE
    use dalek::{EphemeralSecret, PublicKey, SharedSecret};
    let mut public_key_bytes = [0u8; 32];
    PublicRng.fill_bytes(&mut public_key_bytes);

    let public = PublicKey::from(public_key_bytes);
    let ephemeral = EphemeralSecret::random_from_rng(PrivateRng);
    let shared_secret = ephemeral.diffie_hellman(&public);
    core::hint::black_box(shared_secret);
}
