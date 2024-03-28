use crate::rng::{RngCore, CryptoRng, PublicRng, PrivateRng};

pub fn checkct() {
    use vulnerable_eq::eq;
    let mut left = [0u8; 32];
    let mut right = [0u8; 32];
    PrivateRng.fill_bytes(&mut left);
    PublicRng.fill_bytes(&mut right);

    eq(&left, &right);// USER CODE GOES HERE
}