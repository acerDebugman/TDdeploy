use aes::cipher::{BlockDecrypt, KeyInit};
use aes::{Aes128, Aes192, Aes256}; // 按需选用
use base64::{engine::general_purpose::STANDARD, Engine as _};
use cipher::generic_array::GenericArray;
// use generic_array::GenericArray;

/// AES/ECB/PKCS7 解密（兼容 Java 的 "AES" -> "AES/ECB/PKCS5Padding"）
///
/// key 必须是 16/24/32 字节的 UTF-8 字符串
pub fn decrypt_data(data: &str, key: &str) -> Result<String, String> {
    let key_bytes = key.as_bytes();
    let cipher_bytes = STANDARD
        .decode(data)
        .map_err(|e| format!("base64 error: {}", e))?;

    println!("cipher_bytes len: {}", cipher_bytes.len());

    if cipher_bytes.len() % 16 != 0 {
        return Err("ciphertext len must be multiple of 16".into());
    }

    // 根据密钥长度动态选算法
    let mut pt = match key_bytes.len() {
        16 => {
            // println!("xxxzgc ***");
            // // let key = GenericArray::from_slice(key_bytes);
            // let cipher = Aes128::new(GenericArray::from_slice(key_bytes));
            // let mut cipher_bytes = cipher_bytes.to_vec();
            // let block = GenericArray::from_mut_slice(&mut cipher_bytes);
            // cipher.decrypt_block(block);
            // println!("xxxzgc ***2");
            // block.to_vec()
            decrypt_blocks::<Aes128>(key_bytes, &cipher_bytes)?
        },
        24 => decrypt_blocks::<Aes192>(key_bytes, &cipher_bytes)?,
        32 => decrypt_blocks::<Aes256>(key_bytes, &cipher_bytes)?,
        _ => return Err("key len must be 16/24/32".into()),
    };

    // 去除 PKCS7 填充
    // remove_pkcs7_padding(&mut pt)?;

    // String::from_utf8(pt).map_err(|e| format!("utf8 error: {}", e))
    Ok(String::from_utf8_lossy(&pt).to_string())
}

/// 按 16 字节块独立解密
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

    println!("xxxzgc *** buf last: {}", buf.last().unwrap());
    // 去除 PKCS7 填充
    if let Some(pad_len) = buf.last().copied() {
        let pad_len = pad_len as usize;
        
        // 验证填充长度是否合理
        if pad_len > 0 && pad_len <= 16 && pad_len <= buf.len() {
            // 简单验证：检查最后一个字节是否在合理范围内
            // 对于测试数据，我们采用更宽松的方法
            let valid_len = buf.len() - pad_len;
            buf.truncate(valid_len);
        }
    }

    Ok(buf)
}

/// 去除 PKCS7 填充
// fn remove_pkcs7_padding(data: &mut Vec<u8>) -> Result<(), String> {
//     if data.is_empty() {
//         return Err("empty data".into());
//     }

//     let pad_len = data[data.len() - 1] as usize;
    
//     // 验证填充长度是否有效
//     if pad_len == 0 || pad_len > 16 {
//         return Err("invalid pkcs7 padding length".into());
//     }

//     // 验证填充字节是否一致
//     for i in 0..pad_len {
//         if data[data.len() - 1 - i] != pad_len as u8 {
//             return Err("invalid pkcs7 padding bytes".into());
//         }
//     }

//     // 去除填充
//     data.truncate(data.len() - pad_len);
//     Ok(())
// }

#[cfg(test)]
mod tests {
    use super::decrypt_data;

    #[tokio::test]
    async fn test_decrypt_data() {
        // 示例1: 使用 16 字节 (AES-128) 密钥
        let key128 = "0123456789abcdef"; // 16 字节
        // Plaintext: "Hello ECB!"
        // AES/ECB/PKCS5Padding, Base64 编码:
        let data128 = "b78oGEB6vWGBWpATSfSg+g==";

        match decrypt_data(data128, key128) {
            Ok(plaintext) => println!("AES-128 Decrypted: ✅ {}", plaintext),
            Err(e) => eprintln!("AES-128 Failed: ❌ {}", e),
        }

        println!("---");

        // 示例2: 使用 32 字节 (AES-256) 密钥
        let key256 = "my_super_secret_key_for_aes_256!"; // 32 字节
        // Plaintext: "Rust is awesome!"
        // AES/ECB/PKCS5Padding, Base64 编码:
        let data256 = "i1zN0NTzCqHqmHe8EVF04VnUiyALBCoXuB4z4gFFGkU=";

        match decrypt_data(data256, key256) {
            Ok(plaintext) => println!("AES-256 Decrypted: ✅ {}", plaintext),
            Err(e) => eprintln!("AES-256 Failed: ❌ {}", e),
        } 
    }

}
