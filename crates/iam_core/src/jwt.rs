//! JWT 签发与校验模块
//! 
//! 实现 JWT (JSON Web Token) 的签发与校验功能。

use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{AuthError, AuthResult};
use crate::rbac::Role;

/// JWT Claims
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (用户 ID)
    pub sub: String,
    
    /// Issuer (签发者)
    pub iss: String,
    
    /// Audience (接收者)
    pub aud: String,
    
    /// 过期时间
    pub exp: u64,
    
    /// 签发时间
    pub iat: u64,
    
    /// JWT ID
    pub jti: String,
    
    /// 用户角色
    pub role: String,
    
    /// 自定义数据
    pub data: Option<serde_json::Value>,
}

impl Claims {
    /// 创建新的 Claims
    pub fn new(
        user_id: impl Into<String>,
        issuer: impl Into<String>,
        audience: impl Into<String>,
        role: Role,
        expiry_hours: i64,
    ) -> Self {
        let now = Utc::now();
        let exp = now + Duration::hours(expiry_hours);
        
        Self {
            sub: user_id.into(),
            iss: issuer.into(),
            aud: audience.into(),
            exp: exp.timestamp() as u64,
            iat: now.timestamp() as u64,
            jti: Uuid::new_v4().to_string(),
            role: role.to_string(),
            data: None,
        }
    }
    
    /// 设置自定义数据
    pub fn with_data(mut self, data: serde_json::Value) -> Self {
        self.data = Some(data);
        self
    }
    
    /// 检查是否已过期
    pub fn is_expired(&self) -> bool {
        let now = Utc::now().timestamp() as u64;
        self.exp < now
    }
    
    /// 获取过期时间
    pub fn expires_at(&self) -> DateTime<Utc> {
        DateTime::from_timestamp(self.exp as i64, 0).unwrap_or_default()
    }
    
    /// 获取签发时间
    pub fn issued_at(&self) -> DateTime<Utc> {
        DateTime::from_timestamp(self.iat as i64, 0).unwrap_or_default()
    }
    
    /// 获取角色
    pub fn get_role(&self) -> Option<Role> {
        Role::from_str(&self.role)
    }
}

/// JWT 配置
#[derive(Debug, Clone)]
pub struct JwtConfig {
    /// 签发者
    pub issuer: String,
    
    /// 接收者
    pub audience: String,
    
    /// 过期时间（小时）
    pub expiry_hours: i64,
    
    /// 刷新令牌过期时间（小时）
    pub refresh_expiry_hours: i64,
}

impl Default for JwtConfig {
    fn default() -> Self {
        Self {
            issuer: "synapse-core".to_string(),
            audience: "synapse-core-api".to_string(),
            expiry_hours: 24,
            refresh_expiry_hours: 720, // 30 天
        }
    }
}

/// JWT 服务
pub struct JwtService {
    /// 配置
    config: JwtConfig,
    
    /// 签名密钥（简化版本，生产环境应使用更安全的密钥管理）
    secret: String,
}

impl JwtService {
    /// 创建新的 JWT 服务
    pub fn new(config: JwtConfig, secret: impl Into<String>) -> Self {
        Self {
            config,
            secret: secret.into(),
        }
    }
    
    /// 签发访问令牌
    pub fn sign_token(&self, user_id: &str, role: Role) -> AuthResult<String> {
        let claims = Claims::new(
            user_id,
            &self.config.issuer,
            &self.config.audience,
            role,
            self.config.expiry_hours,
        );
        
        // 简化版本：Base64 编码
        // 生产环境应使用 HMAC-SHA256 或 RSA 签名
        let payload = serde_json::to_string(&claims)
            .map_err(|e| AuthError::InternalError(e.to_string()))?;
        
        let encoded = base64_encode(payload.as_bytes());
        let signature = self.sign(&encoded);
        
        Ok(format!("{}.{}.{}", "eyJhbGciOiJIUzI1NiJ9", encoded, signature))
    }
    
    /// 签发刷新令牌
    pub fn sign_refresh_token(&self, user_id: &str, role: Role) -> AuthResult<String> {
        let claims = Claims::new(
            user_id,
            &self.config.issuer,
            &self.config.audience,
            role,
            self.config.refresh_expiry_hours,
        );
        
        let payload = serde_json::to_string(&claims)
            .map_err(|e| AuthError::InternalError(e.to_string()))?;
        
        let encoded = base64_encode(payload.as_bytes());
        let signature = self.sign(&encoded);
        
        Ok(format!("{}.{}.{}", "eyJhbGciOiJIUzI1NiJ9", encoded, signature))
    }
    
    /// 校验令牌
    pub fn verify_token(&self, token: &str) -> AuthResult<Claims> {
        let parts: Vec<&str> = token.split('.').collect();
        
        if parts.len() != 3 {
            return Err(AuthError::InvalidToken("Invalid token format".to_string()));
        }
        
        // 验证签名
        let expected_signature = self.sign(parts[1]);
        if parts[2] != expected_signature {
            return Err(AuthError::InvalidToken("Invalid signature".to_string()));
        }
        
        // 解码 payload
        let payload = base64_decode(parts[1])
            .map_err(|e| AuthError::InvalidToken(e))?;
        
        let claims: Claims = serde_json::from_slice(&payload)
            .map_err(|e| AuthError::InvalidToken(e.to_string()))?;
        
        // 检查是否过期
        if claims.is_expired() {
            return Err(AuthError::TokenExpired);
        }
        
        // 检查 issuer
        if claims.iss != self.config.issuer {
            return Err(AuthError::InvalidToken("Invalid issuer".to_string()));
        }
        
        // 检查 audience
        if claims.aud != self.config.audience {
            return Err(AuthError::InvalidToken("Invalid audience".to_string()));
        }
        
        Ok(claims)
    }
    
    /// 刷新令牌
    pub fn refresh_token(&self, refresh_token: &str) -> AuthResult<(String, String)> {
        let claims = self.verify_token(refresh_token)?;
        
        // 检查角色
        let role = claims.get_role()
            .ok_or_else(|| AuthError::InvalidToken("Invalid role".to_string()))?;
        
        // 签发新的令牌对
        let new_access_token = self.sign_token(&claims.sub, role)?;
        let new_refresh_token = self.sign_refresh_token(&claims.sub, role)?;
        
        Ok((new_access_token, new_refresh_token))
    }
    
    /// 签名数据
    fn sign(&self, data: &str) -> String {
        // 简化版本：使用 HMAC-SHA256
        // 生产环境应使用 ring 或其他安全的加密库
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        self.secret.hash(&mut hasher);
        
        format!("{:x}", hasher.finish())
    }
}

/// Base64 编码（简化版本）
fn base64_encode(data: &[u8]) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    
    let mut result = String::new();
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        
        let triple = (b0 << 16) | (b1 << 8) | b2;
        
        result.push(ALPHABET[((triple >> 18) & 0x3F) as usize] as char);
        result.push(ALPHABET[((triple >> 12) & 0x3F) as usize] as char);
        
        if chunk.len() > 1 {
            result.push(ALPHABET[((triple >> 6) & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
        
        if chunk.len() > 2 {
            result.push(ALPHABET[(triple & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
    }
    
    result
}

/// Base64 解码（简化版本）
fn base64_decode(data: &str) -> Result<Vec<u8>, String> {
    use std::collections::HashMap;
    
    let alphabet: HashMap<char, u8> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
        .chars()
        .enumerate()
        .map(|(i, c)| (c, i as u8))
        .collect();
    
    let data = data.trim_end_matches('=');
    let mut result = Vec::new();
    
    let bytes: Vec<u8> = data.chars().filter_map(|c| alphabet.get(&c).copied()).collect();
    
    for chunk in bytes.chunks(4) {
        if chunk.len() < 2 {
            continue;
        }
        
        let b0 = chunk[0] as u32;
        let b1 = chunk[1] as u32;
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let b3 = if chunk.len() > 3 { chunk[3] as u32 } else { 0 };
        
        let triple = (b0 << 18) | (b1 << 12) | (b2 << 6) | b3;
        
        result.push((triple >> 16) as u8);
        if chunk.len() > 2 {
            result.push((triple >> 8) as u8);
        }
        if chunk.len() > 3 {
            result.push(triple as u8);
        }
    }
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claims_creation() {
        let claims = Claims::new("user1", "test-issuer", "test-audience", Role::User, 24);
        
        assert_eq!(claims.sub, "user1");
        assert_eq!(claims.iss, "test-issuer");
        assert_eq!(claims.aud, "test-audience");
        assert_eq!(claims.role, "user");
        assert!(!claims.is_expired());
    }

    #[test]
    fn test_jwt_service() {
        let config = JwtConfig::default();
        let service = JwtService::new(config, "test-secret");
        
        // 签发令牌
        let token = service.sign_token("user1", Role::User).unwrap();
        assert!(!token.is_empty());
        
        // 校验令牌
        let claims = service.verify_token(&token).unwrap();
        assert_eq!(claims.sub, "user1");
        assert_eq!(claims.role, "user");
    }

    #[test]
    fn test_token_refresh() {
        let config = JwtConfig::default();
        let service = JwtService::new(config, "test-secret");
        
        // 签发刷新令牌
        let refresh_token = service.sign_refresh_token("user1", Role::User).unwrap();
        
        // 刷新令牌
        let (new_access, new_refresh) = service.refresh_token(&refresh_token).unwrap();
        
        assert!(!new_access.is_empty());
        assert!(!new_refresh.is_empty());
    }

    #[test]
    fn test_base64() {
        let data = b"Hello, World!";
        let encoded = base64_encode(data);
        let decoded = base64_decode(&encoded).unwrap();
        
        assert_eq!(data.to_vec(), decoded);
    }
}
