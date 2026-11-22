use anyhow::{anyhow, Result};
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Nonce,
};
use sha2::{Digest, Sha256};

pub struct SecureChannel {
    cipher: ChaCha20Poly1305,
}

impl SecureChannel {
    pub fn new(psk_pin: &str) -> Self {
        // Tạo khóa 32 bytes từ mã PIN / mật khẩu qua SHA-256
        let mut hasher = Sha256::new();
        hasher.update(psk_pin.as_bytes());
        let key_bytes = hasher.finalize();

        let cipher = ChaCha20Poly1305::new_from_slice(&key_bytes).expect("Valid 32-byte key");
        Self { cipher }
    }

    pub fn encrypt(&self, plaintext: &[u8], nonce_bytes: &[u8; 12]) -> Result<Vec<u8>> {
        let nonce = Nonce::from_slice(nonce_bytes);
        self.cipher
            .encrypt(nonce, plaintext)
            .map_err(|e| anyhow!("Encryption error: {:?}", e))
    }

    pub fn decrypt(&self, ciphertext: &[u8], nonce_bytes: &[u8; 12]) -> Result<Vec<u8>> {
        let nonce = Nonce::from_slice(nonce_bytes);
        self.cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| anyhow!("Decryption error: {:?}", e))
    }
}
