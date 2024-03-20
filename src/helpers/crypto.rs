use base64::{engine::general_purpose, Engine};
use md5::{Digest, Md5};
use openssl::symm::{Cipher, Crypter, Mode};
use std::str;

pub fn des_cbc_encrypt(target: &str, key: &str, iv: &str) -> anyhow::Result<String> {
    let _provider = openssl::provider::Provider::try_load(None, "legacy", true).unwrap();
    // Select your cipher directly instead of using `from_name`
    // Example for DES in ECB mode (choose the correct function for your needs)
    let cipher = Cipher::des_cbc();

    let key_bytes = key.as_bytes(); // Consider the appropriate key length and padding
    let iv_bytes = iv.as_bytes(); // Adjust according to your encryption mode

    let mut crypter = Crypter::new(cipher, Mode::Encrypt, key_bytes, Some(iv_bytes))?;

    let data = target.as_bytes(); // Assuming UTF-8 input, adjust if necessary

    let mut encrypted = vec![0; data.len() + cipher.block_size()];
    let count = crypter.update(data, &mut encrypted)?;
    let rest = crypter.finalize(&mut encrypted[count..])?;
    encrypted.truncate(count + rest);

    // Base64 output encoding as an example
    let result = base64::prelude::BASE64_STANDARD.encode(&encrypted);

    Ok(result)
}

pub fn des_cbc_decrypt(target: &str, key: &str, iv: &str) -> anyhow::Result<String> {
    let _provider = openssl::provider::Provider::try_load(None, "legacy", true).unwrap();
    let encrypted_data = general_purpose::STANDARD.decode(target).unwrap();

    let cipher = Cipher::des_cbc();
    let mut decrypter = Crypter::new(cipher, Mode::Decrypt, key.as_bytes(), Some(iv.as_bytes()))?;

    let mut decrypted_data = vec![0; encrypted_data.len() + cipher.block_size()];

    let len = decrypter.update(&encrypted_data, &mut decrypted_data)?;
    let len = len + decrypter.finalize(&mut decrypted_data[len..])?;
    decrypted_data.truncate(len);

    Ok(String::from_utf8(decrypted_data)?)
}

pub fn md5(data: String) -> String {
    let mut hasher = Md5::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}
