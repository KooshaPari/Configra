use aes_gcm::aead::{Aead, KeyInit, OsRng};
use aes_gcm::{AeadCore, Aes256Gcm, Key, Nonce};
use pheno_core::{Error, Result};

pub fn generate_key() -> Vec<u8> {
    use rand::RngCore;
    let mut key = vec![0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut key);
    key
}

pub fn load_key_from_env() -> Result<Vec<u8>> {
    let hex = std::env::var("PHENO_SECRET_KEY")
        .map_err(|_| Error::Crypto("PHENO_SECRET_KEY not set".into()))?;
    hex::decode(&hex).map_err(|e| Error::Crypto(format!("invalid hex key: {e}")))
}

pub fn encrypt(plaintext: &[u8], key: &[u8]) -> Result<(Vec<u8>, Vec<u8>)> {
    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher
        .encrypt(&nonce, plaintext)
        .map_err(|e| Error::Crypto(format!("encrypt failed: {e}")))?;
    Ok((ciphertext, nonce.to_vec()))
}

pub fn decrypt(ciphertext: &[u8], nonce_bytes: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce_bytes);
    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| Error::Crypto(format!("decrypt failed: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_key() {
        let key1 = generate_key();
        let key2 = generate_key();

        // Key should be 32 bytes (256 bits)
        assert_eq!(key1.len(), 32);
        assert_eq!(key2.len(), 32);

        // Keys should be different (with very high probability)
        assert_ne!(key1, key2);
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let key = generate_key();
        let plaintext = b"secret message";

        let (ciphertext, nonce) = encrypt(plaintext, &key).expect("encryption failed");

        assert!(!ciphertext.is_empty());
        assert_eq!(nonce.len(), 12); // GCM nonce is 96 bits (12 bytes)
        assert_ne!(ciphertext, plaintext);

        let decrypted = decrypt(&ciphertext, &nonce, &key).expect("decryption failed");
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_encrypt_decrypt_empty() {
        let key = generate_key();
        let plaintext = b"";

        let (ciphertext, nonce) = encrypt(plaintext, &key).expect("encryption failed");
        let decrypted = decrypt(&ciphertext, &nonce, &key).expect("decryption failed");

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_encrypt_large_plaintext() {
        let key = generate_key();
        let plaintext = vec![42u8; 10000];

        let (ciphertext, nonce) = encrypt(&plaintext, &key).expect("encryption failed");
        let decrypted = decrypt(&ciphertext, &nonce, &key).expect("decryption failed");

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_decrypt_with_wrong_key_fails() {
        let key1 = generate_key();
        let key2 = generate_key();
        let plaintext = b"secret message";

        let (ciphertext, nonce) = encrypt(plaintext, &key1).expect("encryption failed");

        // Decryption with wrong key should fail
        let result = decrypt(&ciphertext, &nonce, &key2);
        assert!(result.is_err());
    }

    #[test]
    fn test_decrypt_with_corrupted_nonce_fails() {
        let key = generate_key();
        let plaintext = b"secret message";

        let (ciphertext, mut nonce) = encrypt(plaintext, &key).expect("encryption failed");

        // Corrupt the nonce
        if !nonce.is_empty() {
            nonce[0] = nonce[0].wrapping_add(1);
        }

        let result = decrypt(&ciphertext, &nonce, &key);
        assert!(result.is_err());
    }

    #[test]
    fn test_decrypt_with_corrupted_ciphertext_fails() {
        let key = generate_key();
        let plaintext = b"secret message";

        let (mut ciphertext, nonce) = encrypt(plaintext, &key).expect("encryption failed");

        // Corrupt the ciphertext
        if !ciphertext.is_empty() {
            ciphertext[0] = ciphertext[0].wrapping_add(1);
        }

        let result = decrypt(&ciphertext, &nonce, &key);
        assert!(result.is_err());
    }

    #[test]
    fn test_encrypt_different_nonces() {
        let key = generate_key();
        let plaintext = b"same plaintext";

        let (ciphertext1, nonce1) = encrypt(plaintext, &key).expect("encryption 1 failed");
        let (ciphertext2, nonce2) = encrypt(plaintext, &key).expect("encryption 2 failed");

        // Nonces should be different
        assert_ne!(nonce1, nonce2);
        // Ciphertexts should be different (due to different nonces)
        assert_ne!(ciphertext1, ciphertext2);

        // But both should decrypt to the same plaintext
        let decrypted1 = decrypt(&ciphertext1, &nonce1, &key).expect("decryption 1 failed");
        let decrypted2 = decrypt(&ciphertext2, &nonce2, &key).expect("decryption 2 failed");

        assert_eq!(decrypted1, plaintext);
        assert_eq!(decrypted2, plaintext);
    }
}
