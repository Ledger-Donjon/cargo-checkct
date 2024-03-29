use crate::rng::{RngCore, CryptoRng, PublicRng, PrivateRng};

pub fn checkct() {
    // USER CODE GOES HERE
    use chachapoly::{
        aead::{AeadCore, AeadInPlace, KeyInit, heapless::Vec},
        ChaCha8Poly1305, Nonce
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