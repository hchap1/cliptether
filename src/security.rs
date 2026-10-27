use std::path::PathBuf;
use bytes::Bytes;

use crate::error::Res;

use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    XChaCha20Poly1305, XNonce,
};

pub fn encrypt(key: &[u8; 32], plaintext: &[u8]) -> Bytes {
    let cipher = XChaCha20Poly1305::new(key.into());
    let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);
    let ciphertext = cipher.encrypt(&nonce, plaintext).unwrap();
    Bytes::from_owner([nonce.as_slice(), &ciphertext].concat())
}

pub fn decrypt(key: &[u8; 32], packet: &[u8]) -> Option<Vec<u8>> {
    if packet.len() < 24 {
        return None;
    }
    let (nonce, ciphertext) = packet.split_at(24);
    let cipher = XChaCha20Poly1305::new(key.into());
    cipher.decrypt(XNonce::from_slice(nonce), ciphertext).ok()
}

pub async fn load_key(path: PathBuf) -> Res<[u8; 32]> {
    let bytes = tokio::fs::read(path).await?;
    bytes.try_into().map_err(|_| crate::error::Error::InvalidKey)   
}
