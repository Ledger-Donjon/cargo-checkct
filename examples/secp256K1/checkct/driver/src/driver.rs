use crate::rng::{CryptoRng, PrivateRng, PublicRng, RngCore};
use checkct_macros::checkct;

#[checkct]
pub fn checkct() {
    // USER CODE GOES HERE
    use secp256K1::{AlignedType, Message, PublicKey, Secp256k1, SecretKey};

    let mut buf = [AlignedType::zeroed(); 512];
    let Ok(secp) = Secp256k1::preallocated_new(&mut buf) else {
        return;
    };

    let mut secret_key = [0u8; 32];
    PrivateRng.fill_bytes(&mut secret_key);
    let Ok(secret_key) = SecretKey::from_slice(&secret_key) else {
        return;
    };

    let public_key = PublicKey::from_secret_key(&secp, &secret_key);
    let mut message = [0u8; 32];
    PublicRng.fill_bytes(&mut message);
    let Ok(message) = Message::from_digest_slice(&message) else {
        return;
    };

    let sig = secp.sign_ecdsa(&message, &secret_key);
    core::hint::black_box(sig);
}
