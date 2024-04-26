#![no_std]
#![no_main]

use checkct::{PrivateRng, PublicRng, RngCore};

#[no_mangle]
fn main() {
    use chacha20poly1305::{
        aead::{AeadCore, AeadInPlace, KeyInit, heapless::Vec},
        ChaCha20Poly1305
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
