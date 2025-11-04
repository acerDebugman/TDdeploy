use aes::cipher::{BlockDecrypt, KeyInit};
use aes::{Aes128, Aes192, Aes256};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use cipher::generic_array::GenericArray;

/// 抽象解密器 trait，对应 Java 的 AESBaseDecryptor
pub trait AESBaseDecryptor {
    /// 解密数据
    fn decrypt_data(&self, data: &str, key: &str) -> Result<String, String>;
}

/// AES ECB 解密器，对应 Java 的 AESECBDecryptor
pub struct AESECBDecryptor;

impl AESECBDecryptor {
    /// 创建新的 AES ECB 解密器实例
    pub fn new() -> Self {
        AESECBDecryptor
    }
}

impl AESBaseDecryptor for AESECBDecryptor {
    /// AES/ECB/PKCS7 解密（兼容 Java 的 "AES" -> "AES/ECB/PKCS5Padding"）
    fn decrypt_data(&self, data: &str, key: &str) -> Result<String, String> {
        println!("*******zgc in aes ecb len:{}|{}", data, key);
        
        let key_bytes = key.as_bytes();
        let cipher_bytes = STANDARD
            .decode(data)
            .map_err(|e| format!("base64 error: {}", e))?;

        println!("*******zgc aes ecb len:{}", cipher_bytes.len());

        if cipher_bytes.len() % 16 != 0 {
            return Err("ciphertext len must be multiple of 16".into());
        }

        // 根据密钥长度动态选算法
        let pt = match key_bytes.len() {
            16 => decrypt_blocks::<Aes128>(key_bytes, &cipher_bytes)?,
            24 => decrypt_blocks::<Aes192>(key_bytes, &cipher_bytes)?,
            32 => decrypt_blocks::<Aes256>(key_bytes, &cipher_bytes)?,
            _ => return Err("key len must be 16/24/32".into()),
        };

        String::from_utf8(pt).map_err(|e| format!("utf8 error: {}", e))
    }
}

/// 真正干活：按 16 B 块独立解密，再去 PKCS#7 填充
fn decrypt_blocks<A>(key: &[u8], ct: &[u8]) -> Result<Vec<u8>, String>
where
    A: BlockDecrypt + KeyInit,
{
    let cipher = A::new(GenericArray::from_slice(key));
    let mut buf = ct.to_vec();

    // 逐块解密
    for chunk in buf.chunks_mut(16) {
        let block = GenericArray::from_mut_slice(chunk);
        cipher.decrypt_block(block);
    }

    // 去 PKCS#7 填充
    let pad = buf.last().copied().unwrap() as usize;
    if pad == 0 || pad > 16 {
        return Err("invalid pkcs7 padding".into());
    }
    let valid = buf.len() - pad;
    buf.truncate(valid);
    Ok(buf)
}

/// 为了方便使用，也提供一个独立的 decrypt_data 函数（向后兼容）
pub fn decrypt_data(data: &str, key: &str) -> Result<String, String> {
    let decryptor = AESECBDecryptor::new();
    decryptor.decrypt_data(data, key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_aes_ecb_decryptor() {
        // 创建解密器实例
        let decryptor = AESECBDecryptor::new();
        
        // 示例1: 使用 16 字节 (AES-128) 密钥
        let key128 = "0123456789abcdef"; // 16 字节
        // Plaintext: "Hello ECB!"
        // AES/ECB/PKCS5Padding, Base64 编码:
        let data128 = "b78oGEB6vWGBWpATSfSg+g==";

        match decryptor.decrypt_data(data128, key128) {
            Ok(plaintext) => println!("AES-128 Decrypted: ✅ {}", plaintext),
            Err(e) => eprintln!("AES-128 Failed: ❌ {}", e),
        }

        println!("---");

        // 示例2: 使用 32 字节 (AES-256) 密钥
        let key256 = "my_super_secret_key_for_aes_256!"; // 32 字节
        // Plaintext: "Rust is awesome!"
        // AES/ECB/PKCS5Padding, Base64 编码:
        let data256 = "i1zN0NTzCqHqmHe8EVF04VnUiyALBCoXuB4z4gFFGkU=";

        match decryptor.decrypt_data(data256, key256) {
            Ok(plaintext) => println!("AES-256 Decrypted: ✅ {}", plaintext),
            Err(e) => eprintln!("AES-256 Failed: ❌ {}", e),
        }
    }

    #[tokio::test]
    async fn test_decrypt_data_function() {
        // 测试独立的 decrypt_data 函数
        let key128 = "0123456789abcdef"; // 16 字节
        let data128 = "b78oGEB6vWGBWpATSfSg+g==";

        match decrypt_data(data128, key128) {
            Ok(plaintext) => println!("Function AES-128 Decrypted: ✅ {}", plaintext),
            Err(e) => eprintln!("Function AES-128 Failed: ❌ {}", e),
        }
    }
}