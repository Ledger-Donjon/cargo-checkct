use crate::rng::{CryptoRng, PrivateRng, PublicRng, RngCore};
use checkct_macros::checkct;

#[checkct]
pub fn checkct() {
    use subtle_eq::ConstantTimeEq;
    let mut left = [0u8; 8];
    let mut right = [0x42u8; 8];
    PrivateRng.fill_bytes(&mut left);
    PublicRng.fill_bytes(&mut right);

    left.ct_eq(&right);// USER CODE GOES HERE
}
