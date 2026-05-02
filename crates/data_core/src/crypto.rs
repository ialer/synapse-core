//! 加密核心模块
//! 
//! 基于 ring 库实现 AES-256-GCM 加解密功能。
//! 
//! # 安全设计
//! 
//! - 使用 AES-256-GCM 认证加密
//! - 每次加密生成唯一 Nonce
//! - 支持关联数据 (Associated Data)
//! - 密钥派生使用 HKDF

use ring::aead;
use ring::rand::{SecureRandom, SystemRandom};
use thiserror::Error;

/// 加密错误类型
#[derive(Debug, Error)]
pub enum CipherError {
    /// 密钥生成失败
    #[error("密钥生成失败: {0}")]
    KeyGenerationFailed(String),
    
    /// 加密失败
    #[error("加密失败: {0}")]
    EncryptionFailed(String),
    
    /// 解密失败
    #[error("解密失败: {0}")]
    DecryptionFailed(String),
    
    /// 无效的密钥长度
    #[error("无效的密钥长度: {0} (需要 32 字节)")]
    InvalidKeyLength(usize),
    
    /// 无效的 Nonce 长度
    #[error("无效的 Nonce 长度: {0} (需要 12 字节)")]
    InvalidNonceLength(usize),
    
    /// 数据完整性验证失败
    #[error("数据完整性验证失败 (认证标签不匹配)")]
    IntegrityCheckFailed,
}

/// AES-256-GCM 加解密器
/// 
/// 提供安全的对称加密功能，支持：
/// - 密钥生成与管理
/// - 数据加密与解密
/// - 关联数据支持 (AEAD)
/// 
/// # 示例
/// 
/// ```rust
/// use data_core::Cipher;
/// 
/// // 创建加密器并生成新密钥
/// let cipher = Cipher::new().expect("密钥生成失败");
/// 
/// // 加密数据
/// let plaintext = b"my secret data";
/// let ciphertext = cipher.encrypt(plaintext, None).expect("加密失败");
/// 
/// // 解密数据
/// let decrypted = cipher.decrypt(&ciphertext, None).expect("解密失败");
/// assert_eq!(plaintext.to_vec(), decrypted);
/// ```
#[derive(Clone)]
pub struct Cipher {
    /// 加密密钥
    key: aead::LessSafeKey,
    /// 随机数生成器
    rng: SystemRandom,
}

impl Cipher {
    /// 密钥长度 (AES-256 = 32 字节)
    pub const KEY_LENGTH: usize = 32;
    
    /// Nonce 长度 (AES-GCM = 12 字节)
    pub const NONCE_LENGTH: usize = 12;
    
    /// 认证标签长度 (AES-GCM = 16 字节)
    pub const TAG_LENGTH: usize = 16;

    /// 创建新的加密器并生成随机密钥
    /// 
    /// # 返回
    /// 
    /// 新创建的加密器实例
    pub fn new() -> Result<Self, CipherError> {
        let rng = SystemRandom::new();
        
        // 生成随机密钥
        let key_bytes = {
            let mut buf = [0u8; Self::KEY_LENGTH];
            rng.fill(&mut buf)
                .map_err(|e| CipherError::KeyGenerationFailed(e.to_string()))?;
            buf
        };
        
        let key = aead::UnboundKey::new(&aead::AES_256_GCM, &key_bytes)
            .map_err(|e| CipherError::KeyGenerationFailed(e.to_string()))?;
        
        Ok(Self {
            key: aead::LessSafeKey::new(key),
            rng,
        })
    }

    /// 使用现有密钥创建加密器
    /// 
    /// # 参数
    /// 
    /// * `key_bytes` - 32 字节的密钥
    /// 
    /// # 返回
    /// 
    /// 加密器实例
    pub fn with_key(key_bytes: &[u8]) -> Result<Self, CipherError> {
        if key_bytes.len() != Self::KEY_LENGTH {
            return Err(CipherError::InvalidKeyLength(key_bytes.len()));
        }
        
        let key = aead::UnboundKey::new(&aead::AES_256_GCM, key_bytes)
            .map_err(|e| CipherError::KeyGenerationFailed(e.to_string()))?;
        
        Ok(Self {
            key: aead::LessSafeKey::new(key),
            rng: SystemRandom::new(),
        })
    }

    /// 从密码派生密钥
    /// 
    /// 使用 HKDF 从密码派生出 AES-256 密钥。
    /// 生产环境建议使用 Argon2id。
    /// 
    /// # 参数
    /// 
    /// * `password` - 用户密码
    /// * `salt` - 盐值 (建议 16 字节)
    /// 
    /// # 返回
    /// 
    /// 加密器实例
    pub fn from_password(password: &[u8], salt: &[u8]) -> Result<Self, CipherError> {
        // 使用 ring 的 HKDF 从密码派生密钥
        let prk = ring::hkdf::Prk::new_less_safe(
            ring::hkdf::HKDF_SHA256,
            password,
        );
        
        let mut key_bytes = [0u8; Self::KEY_LENGTH];
        prk.expand(&[salt], ring::hkdf::HKDF_SHA256)
            .map_err(|e| CipherError::KeyGenerationFailed(e.to_string()))?
            .fill(&mut key_bytes)
            .map_err(|e| CipherError::KeyGenerationFailed(e.to_string()))?;
        
        Self::with_key(&key_bytes)
    }

    /// 生成随机 Nonce
    fn generate_nonce(&self) -> Result<aead::Nonce, CipherError> {
        let mut nonce_bytes = [0u8; Self::NONCE_LENGTH];
        self.rng.fill(&mut nonce_bytes)
            .map_err(|e| CipherError::EncryptionFailed(e.to_string()))?;
        
        Ok(aead::Nonce::assume_unique_for_key(nonce_bytes))
    }

    /// 加密数据
    /// 
    /// # 参数
    /// 
    /// * `plaintext` - 待加密的明文数据
    /// * `aad` - 关联数据 (可选，不加密但参与认证)
    /// 
    /// # 返回
    /// 
    /// 加密后的数据 (Nonce + 密文 + 认证标签)
    /// 
    /// # 格式
    /// 
    /// ```text
    /// [Nonce (12 bytes)] [Ciphertext] [Tag (16 bytes)]
    /// ```
    /// 
    /// # 示例
    /// 
    /// ```rust
    /// use data_core::crypto::Cipher;
    /// let cipher = Cipher::new().unwrap();
    /// let ciphertext = cipher.encrypt(b"secret", None).unwrap();
    /// ```
    pub fn encrypt(&self, plaintext: &[u8], aad: Option<&[u8]>) -> Result<Vec<u8>, CipherError> {
        let nonce = self.generate_nonce()?;
        let nonce_bytes = nonce.as_ref().to_vec(); // 保存 nonce 字节
        
        // 构建密文缓冲区 (需要额外空间存放认证标签)
        let mut in_out = plaintext.to_vec();
        
        // 执行加密
        let associated_data = aad.unwrap_or(&[]);
        let seal_in_place_result = self.key.seal_in_place_append_tag(
            nonce,
            aead::Aad::from(associated_data),
            &mut in_out,
        );
        
        seal_in_place_result
            .map_err(|e| CipherError::EncryptionFailed(e.to_string()))?;
        
        // 构建最终输出: Nonce + Ciphertext + Tag
        let mut output = Vec::with_capacity(Self::NONCE_LENGTH + in_out.len());
        output.extend_from_slice(&nonce_bytes);
        output.extend_from_slice(&in_out);
        
        Ok(output)
    }

    /// 解密数据
    /// 
    /// # 参数
    /// 
    /// * `ciphertext` - 加密数据 (Nonce + 密文 + 认证标签)
    /// * `aad` - 关联数据 (必须与加密时一致)
    /// 
    /// # 返回
    /// 
    /// 解密后的明文数据
    /// 
    /// # 错误
    /// 
    /// - 数据被篡改 (认证标签不匹配)
    /// - 格式错误
    pub fn decrypt(&self, ciphertext: &[u8], aad: Option<&[u8]>) -> Result<Vec<u8>, CipherError> {
        // 检查最小长度
        if ciphertext.len() < Self::NONCE_LENGTH + Self::TAG_LENGTH {
            return Err(CipherError::DecryptionFailed(
                "数据长度不足".to_string()
            ));
        }
        
        // 提取 Nonce
        let (nonce_bytes, encrypted_data) = ciphertext.split_at(Self::NONCE_LENGTH);
        let nonce = aead::Nonce::try_assume_unique_for_key(nonce_bytes)
            .map_err(|e| CipherError::DecryptionFailed(e.to_string()))?;
        
        // 准备解密缓冲区
        let mut in_out = encrypted_data.to_vec();
        
        // 执行解密
        let associated_data = aad.unwrap_or(&[]);
        let open_in_place_result = self.key.open_in_place(
            nonce,
            aead::Aad::from(associated_data),
            &mut in_out,
        );
        
        match open_in_place_result {
            Ok(plaintext) => Ok(plaintext.to_vec()),
            Err(_) => Err(CipherError::IntegrityCheckFailed),
        }
    }

    /// 获取密钥的字节表示 (用于持久化)
    /// 
    /// # 安全警告
    /// 
    /// 此方法返回原始密钥，应安全存储。
    pub fn key_bytes(&self) -> Vec<u8> {
        // 注意：ring 的 LessSafeKey 不直接暴露密钥
        // 这里需要在创建时保存密钥副本
        // 生产环境应使用安全的密钥存储
        Vec::new() // 占位实现
    }
}

/// 密钥包装器 (用于安全存储)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WrappedKey {
    /// 加密后的密钥
    pub encrypted_key: Vec<u8>,
    /// 密钥版本 (用于密钥轮换)
    pub version: u32,
    /// 创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,
}

use serde::{Deserialize, Serialize};

impl WrappedKey {
    /// 包装密钥
    pub fn wrap(key: &[u8], wrapping_key: &Cipher) -> Result<Self, CipherError> {
        let encrypted_key = wrapping_key.encrypt(key, Some(b"key-wrap"))?;
        Ok(Self {
            encrypted_key,
            version: 1,
            created_at: chrono::Utc::now(),
        })
    }

    /// 解包密钥
    pub fn unwrap(&self, wrapping_key: &Cipher) -> Result<Vec<u8>, CipherError> {
        wrapping_key.decrypt(&self.encrypted_key, Some(b"key-wrap"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cipher_creation() {
        let cipher = Cipher::new();
        assert!(cipher.is_ok());
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let cipher = Cipher::new().unwrap();
        let plaintext = b"Hello, SynapseCore!";
        
        // 加密
        let ciphertext = cipher.encrypt(plaintext, None).unwrap();
        assert_ne!(plaintext.to_vec(), ciphertext);
        
        // 解密
        let decrypted = cipher.decrypt(&ciphertext, None).unwrap();
        assert_eq!(plaintext.to_vec(), decrypted);
    }

    #[test]
    fn test_encrypt_with_aad() {
        let cipher = Cipher::new().unwrap();
        let plaintext = b"secret data";
        let aad = b"additional context";
        
        let ciphertext = cipher.encrypt(plaintext, Some(aad)).unwrap();
        let decrypted = cipher.decrypt(&ciphertext, Some(aad)).unwrap();
        
        assert_eq!(plaintext.to_vec(), decrypted);
    }

    #[test]
    fn test_tampered_data_detection() {
        let cipher = Cipher::new().unwrap();
        let plaintext = b"important data";
        
        let mut ciphertext = cipher.encrypt(plaintext, None).unwrap();
        
        // 篡改数据
        if let Some(byte) = ciphertext.last_mut() {
            *byte ^= 0xFF;
        }
        
        // 尝试解密应该失败
        let result = cipher.decrypt(&ciphertext, None);
        assert!(matches!(result, Err(CipherError::IntegrityCheckFailed)));
    }

    #[test]
    fn test_key_from_password() {
        let password = b"my secure password";
        let salt = b"unique salt value ";
        
        let cipher = Cipher::from_password(password, salt).unwrap();
        let plaintext = b"protected data";
        
        let ciphertext = cipher.encrypt(plaintext, None).unwrap();
        let decrypted = cipher.decrypt(&ciphertext, None).unwrap();
        
        assert_eq!(plaintext.to_vec(), decrypted);
    }
}
