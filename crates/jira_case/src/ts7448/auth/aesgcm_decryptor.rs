// use aes_gcm::{
//     AeadCore, Aes128Gcm, Aes256Gcm, Key, Nonce, Tag, aead::{Aead, AeadInPlace, KeyInit}
// };
// use base64::{engine::general_purpose::STANDARD, Engine as _};
// use rand::rngs::OsRng;

// /// AES-GCM 解密，与 Java `AES/GCM/NoPadding` 完全等价
// /// key: 16 或 32 字节 UTF-8 字符串  
// /// data: Base64(12B nonce || ciphertext || 16B tag)
// pub fn decrypt_data(data: &str, key: &str) -> anyhow::Result<String> {
//     const NONCE_LEN: usize = 12;
//     const TAG_LEN:  usize = 16;

//     let key_bytes = key.as_bytes();
//     let blob      = STANDARD.decode(data).map_err(|e| anyhow::anyhow!("base64:{e}"))?;
//     if blob.len() < NONCE_LEN + TAG_LEN {
//         return Err(anyhow::anyhow!("blob too short"));
//     }

//     let (nonce_ct, tag_bytes) = blob.split_at(blob.len() - TAG_LEN);
//     let (nonce_bytes, ct)     = nonce_ct.split_at(NONCE_LEN);

//     // ✅ 修正：让类型系统自己推断 Nonce 长度
//     let nonce = Nonce::from_slice(nonce_bytes);   // 12 B
//     let tag   = Tag::from_slice(tag_bytes);       // 16 B

//     let plain = match key_bytes.len() {
//         16 => decrypt::<Aes128Gcm>(key_bytes, ct, nonce, tag).unwrap(),
//         32 => decrypt::<Aes256Gcm>(key_bytes, ct, nonce, tag).unwrap(),
//         _  => return Err(anyhow::anyhow!("key must be 16 or 32 bytes")),
//     };

//     String::from_utf8(plain).map_err(|e| format!("utf8:{e}"))

//     //2
//     // println!("in decrypt key len: {}", key_bytes.len());
//     // let key = Key::<Aes256Gcm>::from_slice(key_bytes);

//     // let cipher = Aes256Gcm::new(&key);
//     // let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
//     // let ciphertext = cipher.encrypt(&nonce, b"plaintext message".as_ref()).unwrap();
//     // let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref());
//     // println!("finale result plaintext: {:?}", plaintext);

//     // Ok(String::from_utf8(plaintext.unwrap())?)
// }

// /// 真正干活：就地解密并剥离 tag
// fn decrypt<A>(key: &[u8], ct: &[u8], nonce: &Nonce<A>, tag: &Tag<A>) -> Result<Vec<u8>, String>
// where
//     A: aes_gcm::AeadInPlace + KeyInit + cipher::ArrayLength<u8>,
// {
//     let cipher = A::new(Key::<A>::from_slice(key));
//     let mut buf = ct.to_vec();
//     buf.extend_from_slice(tag);          // 把 tag 拼到末尾
//     cipher
//         .decrypt_in_place(nonce, b"", &mut buf)
//         .map_err(|_| "aes-gcm decrypt failed".to_string())?;
//     buf.truncate(ct.len());              // 去掉 tag
//     Ok(buf)
// }

use aes_gcm::aead::{Aead, KeyInit, Nonce};
use aes_gcm::{Aes128Gcm, Aes256Gcm};
use base64::{engine::general_purpose::STANDARD as base64_engine, Engine as _};
use std::string::FromUtf8Error;

// --- 模仿 Java 代码中的常量 ---
const GCM_NONCE_LENGTH: usize = 12;
const GCM_TAG_LENGTH: usize = 16; // 128 bits

/// 定义我们的自定义错误类型，使函数签名更清晰
#[derive(Debug, thiserror::Error)]
pub enum DecryptionError {
    #[error("Base64 decoding failed: {0}")]
    Base64(#[from] base64::DecodeError),
    #[error("Invalid data length: must be at least {0} bytes", GCM_NONCE_LENGTH + GCM_TAG_LENGTH)]
    InvalidLength,
    #[error("Invalid key length: must be 16, 24, or 32 bytes, but got {0}")]
    InvalidKeyLength(usize),
    #[error("Decryption failed (AEAD error): {0}")]
    // Aead(#[from] aes_gcm::Error),
    Aead(aes_gcm::Error),
    #[error("Decrypted data is not valid UTF-8: {0}")]
    Utf8(#[from] FromUtf8Error),
}

/// 对标 Java 中的 `decryptData` 方法
///
/// - `data`: Base64 编码的字符串，格式为: NONCE + CIPHERTEXT + TAG
/// - `key`: UTF-8 字符串格式的密钥
fn decrypt_data(data: &str, key: &str) -> Result<String, DecryptionError> {
    // 打印调试信息，与 Java 代码保持一致
    println!("*******zgc in aes gcm data|key: {}|{}", data, key);

    // Java: Key secretKey = new SecretKeySpec(key.getBytes(), AES_GCM);
    let key_bytes = key.as_bytes();

    // Java: byte[] message = Base64.decodeBase64(data);
    let message = base64_engine.decode(data)?;

    // Java: if (message.length < GCM_NONCE_LENGTH + GCM_TAG_LENGTH) { ... }
    // 我们至少需要一个 nonce 和一个 tag。密文可以为空。
    if message.len() < GCM_NONCE_LENGTH + GCM_TAG_LENGTH {
        return Err(DecryptionError::InvalidLength);
    }

    // --- 核心逻辑：拆分数据 ---
    // Java: GCMParameterSpec(..., message, 0, GCM_NONCE_LENGTH);
    // 1. 提取 Nonce (前 12 字节)
    let (nonce_bytes, ciphertext_and_tag) = message.split_at(GCM_NONCE_LENGTH);

    // 2. 剩余部分是 [CIPHERTEXT | TAG]
    // Java: cipher.doFinal(message, GCM_NONCE_LENGTH, ...);

    // --- 动态分派密钥大小 ---
    // Java 的 SecretKeySpec 会在 init 时根据 key.length 自动选择
    // Rust 中我们需要显式地做这个分派，这更安全。
    let decrypted_bytes = match key_bytes.len() {
        16 => {
            println!("nonce_bytes: {:?}", hex::encode(&nonce_bytes));
            println!("ciphertext_and_tag: {:?}", hex::encode(&ciphertext_and_tag));
            // 128-bit key
            let cipher = Aes128Gcm::new(key_bytes.into());
            let nonce = Nonce::<Aes128Gcm>::from_slice(nonce_bytes);
            cipher.decrypt(nonce, ciphertext_and_tag).map_err(|e| {
                println!("error: {:?}", e);
                DecryptionError::Aead(e)
            })?
        }
        // 24 => {
        //     // 192-bit key
        //     let cipher = Aes192Gcm::new(key_bytes.into());
        //     cipher.decrypt(nonce, ciphertext_and_tag)?
        // }
        32 => {
            println!("nonce_bytes 256: {:?}", hex::encode(&nonce_bytes));
            println!("ciphertext_and_tag 256: {:?}", hex::encode(&ciphertext_and_tag));
            // 256-bit key
            let cipher = Aes256Gcm::new(key_bytes.into());
            let nonce = Nonce::<Aes256Gcm>::from_slice(nonce_bytes);
            cipher.decrypt(nonce, ciphertext_and_tag).map_err(|e| DecryptionError::Aead(e))?
        }
        _ => {
            return Err(DecryptionError::InvalidKeyLength(key_bytes.len()));
        }
    };

    // Java: return new String(decryptData);
    // Rust 会检查 UTF-8 编码，这更健壮
    let decrypted_string = String::from_utf8(decrypted_bytes)?;

    Ok(decrypted_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_decrypt_data() {
        let key = "1234567890abcdef";          // 16 字节
        let b64 = "3iFfq0bKDm2cVWPhkwWTGA=="; // "hello world!" 的 AES-128/ECB/PKCS7 + Base64

        match decrypt_data(b64, key) {
            Ok(txt) => println!("decrypted: {}", txt),
            Err(e) => eprintln!("error: {}", e),
        }
    }

    #[tokio::test]
    async fn test_tuya() -> anyhow::Result<()> {
        let key = "62cc4527a90e7829";
        let data = "qzGAKa00XbTK4HPjElvpkuYN/fXYyj2BhdTbE6+l6cONQLctSXTwwsSlkrNo+30mlJQeaNZ73vh/NuSeyf4HQgmHrdb3bonBWkxjdbD+bGrDUAr77zAj2RUTyR8inKwqJWaSfnva4UEUW2xRUfWCTRYjyyLJsHO5m8Plg+lW8q5Rg83yEPQniHi1UjEOL34c7fz88PBaNm7MD+5deyG4czT4ZsO+VpwZ2yB6CXDwgGtZhspEHF6EaiNvzo+Rxr0kL+UW+f/dmCkGjxmcHlqpDqdUrrI0ZPc=";

        println!("data len: {}", data.len());
        println!("key len: {}", key.len());
        match decrypt_data(data, key) {
            Ok(txt) => println!("decrypted: {}", txt),
            Err(e) => eprintln!("error: {}", e),
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_data2() -> anyhow::Result<()> {
        // TestVector {
        //     key: &hex!("7fddb57453c241d03efbed3ac44e371c"),
        //     nonce: &hex!("ee283a3fc75575e33efd4887"),
        //     plaintext: &hex!("d5de42b461646c255c87bd2962d3b9a2"),
        //     aad: &hex!(""),
        //     ciphertext: &hex!("2ccda4a5415cb91e135c2a0f78c9b2fd"),
        //     tag: &hex!("b36d1df9b9d5e596f83e8b7f52971cb3"),
        // },
        // 示例1: 使用 16 字节 (AES-128) 密钥
        let key128 = "7fddb57453c241d03efbed3ac44e371c"; // 16 字节
        
        // 平日: "Hello Rust!"
        // Nonce: "uniquenonce1" (12 字节)
        // Java 格式: Base64(NONCE + CIPHERTEXT + TAG)
        // --- 这是修正后的正确数据 ---
        // let data128 = "dW5pcXVlbm9uY2UxWlIsqD8+qH/fKqYfAwofGwc=";
        let data128 = "ee283a3fc75575e33efd48872ccda4a5415cb91e135c2a0f78c9b2fd";

        match decrypt_data(data128, key128) {
            Ok(plaintext) => println!("AES-128 Decrypted: ✅ {}", plaintext),
            Err(e) => eprintln!("AES-128 Failed: ❌ {}", e),
        }
    
        Ok(())
    }

}
