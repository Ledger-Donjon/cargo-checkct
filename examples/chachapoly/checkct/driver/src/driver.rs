use crate::rng::{CryptoRng, PrivateRng, PublicRng, RngCore};
use checkct_macros::checkct;

#[checkct]
pub fn checkct_chacha20() {
    // USER CODE GOES HERE
    use chachapoly::{
        aead::{heapless::Vec, AeadCore, AeadInPlace, KeyInit},
        ChaCha20Poly1305, Nonce,
    };

    let key = ChaCha20Poly1305::generate_key(&mut PrivateRng);
    let cipher = ChaCha20Poly1305::new(&key);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut PublicRng);
    let mut buffer: Vec<u8, 1024> = Vec::new();
    let mut msg = [0u8; 1008];
    PrivateRng.fill_bytes(&mut msg);
    buffer.extend_from_slice(&msg);
    cipher.encrypt_in_place(&nonce, b"", &mut buffer).unwrap();
}

#[checkct]
pub fn checkct_chacha8() {
    // USER CODE GOES HERE
    use chachapoly::{
        aead::{heapless::Vec, AeadCore, AeadInPlace, KeyInit},
        ChaCha8Poly1305, Nonce,
    };

    let key = ChaCha8Poly1305::generate_key(&mut PrivateRng);
    let cipher = ChaCha8Poly1305::new(&key);
    let nonce = ChaCha8Poly1305::generate_nonce(&mut PublicRng);
    let mut buffer: Vec<u8, 1024> = Vec::new();
    let mut msg = [0u8; 1008];
    PrivateRng.fill_bytes(&mut msg);
    buffer.extend_from_slice(&msg);
    cipher.encrypt_in_place(&nonce, b"", &mut buffer).unwrap();
}
