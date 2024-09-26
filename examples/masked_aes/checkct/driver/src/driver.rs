use crate::rng::{CryptoRng, PrivateRng, PublicRng, RngCore};
use checkct_macros::checkct;

#[checkct]
pub fn checkct() {
    // USER CODE GOES HERE
    let mut key = [0u8; 16];
    let mut block = [0u8; 16];
    PrivateRng.fill_bytes(&mut key);
    PublicRng.fill_bytes(&mut block);
    let mut aes = masked_aes::MaskedAes::new(key, PrivateRng.next_u32());
    core::hint::black_box(aes.encrypt(block));
}
