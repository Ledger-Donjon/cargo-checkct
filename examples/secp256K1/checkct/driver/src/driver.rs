use crate::rng::{CryptoRng, PrivateRng, PublicRng, RngCore};

pub fn checkct() {
    // USER CODE GOES HERE
    use secp256K1::{Secp256k1, Message};
    use secp256K1::hashes::{sha256, Hash};

    let secp = Secp256k1::new();
    let (secret_key, public_key) = secp.generate_keypair(&mut PrivateRng);
    let digest = sha256::Hash::hash("Hello World!".as_bytes());
    let message = Message::from_digest(digest.to_byte_array());

    let sig = secp.sign_ecdsa(&message, &secret_key);
    core::hint::black_box(sig);
}
