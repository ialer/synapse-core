//! 认证服务模块
//! 
//! 定义 AuthService Trait，提供认证功能的接口。

use std::path::PathBuf;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::error::AuthResult;
use crate::jwt::{Claims, JwtConfig, JwtService};
use crate::rbac::{Role, UserRole};

/// 认证服务 Trait
#[async_trait]
pub trait AuthService: Send + Sync {
    /// 用户登录
    async fn login(&self, username: &str, password: &str) -> AuthResult<LoginResult>;
    
    /// 用户登出
    async fn logout(&self, token: &str) -> AuthResult<()>;
    
    /// 校验令牌
    async fn verify_token(&self, token: &str) -> AuthResult<Claims>;
    
    /// 刷新令牌
    async fn refresh_token(&self, refresh_token: &str) -> AuthResult<RefreshResult>;
    
    /// 获取用户角色
    async fn get_user_role(&self, user_id: &str) -> AuthResult<UserRole>;
    
    /// 检查用户权限
    async fn check_permission(&self, user_id: &str, permission: &str) -> AuthResult<bool>;

    /// 注册新用户
    async fn register(&self, username: &str, password: &str) -> AuthResult<LoginResult>;
}

/// 登录结果
#[derive(Debug, Clone)]
pub struct LoginResult {
    /// 用户 ID
    pub user_id: String,
    
    /// 用户名
    pub username: String,
    
    /// 角色
    pub role: Role,
    
    /// 访问令牌
    pub access_token: String,
    
    /// 刷新令牌
    pub refresh_token: String,
    
    /// 令牌过期时间
    pub expires_in: i64,
}

/// 刷新结果
#[derive(Debug, Clone)]
pub struct RefreshResult {
    /// 新的访问令牌
    pub access_token: String,
    
    /// 新的刷新令牌
    pub refresh_token: String,
    
    /// 令牌过期时间
    pub expires_in: i64,
}

/// 内存认证服务实现（示例）
pub struct MemoryAuthService {
    /// JWT 服务
    jwt_service: JwtService,
    
    /// 用户存储（简化版本）
    users: std::sync::RwLock<Vec<UserRecord>>,
}

/// 用户记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRecord {
    /// 用户 ID
    id: String,
    
    /// 用户名
    username: String,
    
    /// 密码哈希（简化版本）
    password_hash: String,
    
    /// 角色
    role: Role,
    
    /// 是否启用
    enabled: bool,
}

impl MemoryAuthService {
    /// 创建新的内存认证服务
    pub fn new(jwt_config: JwtConfig, secret: impl Into<String>) -> Self {
        let jwt_service = JwtService::new(jwt_config, secret);
        
        Self {
            jwt_service,
            users: std::sync::RwLock::new(Vec::new()),
        }
    }
    
    /// 添加用户
    pub fn add_user(&self, username: &str, password: &str, role: Role) {
        let user = UserRecord {
            id: uuid::Uuid::new_v4().to_string(),
            username: username.to_string(),
            password_hash: password.to_string(), // 简化版本，生产环境应使用 bcrypt
            role,
            enabled: true,
        };
        
        self.users.write().unwrap().push(user);
    }
    
    /// 查找用户
    fn find_user(&self, username: &str) -> Option<UserRecord> {
        self.users.read().unwrap()
            .iter()
            .find(|u| u.username == username)
            .cloned()
    }
    
    /// 获取所有用户（用于持久化）
    pub fn get_users(&self) -> Vec<UserRecord> {
        self.users.read().unwrap().clone()
    }
    
    /// 从持久化数据加载用户（用于启动时恢复）
    pub fn load_users(&self, users: Vec<UserRecord>) {
        let mut current = self.users.write().unwrap();
        *current = users;
    }
}

#[async_trait]
impl AuthService for MemoryAuthService {
    async fn login(&self, username: &str, password: &str) -> AuthResult<LoginResult> {
        let user = self.find_user(username)
            .ok_or_else(|| crate::error::AuthError::UserNotFound(username.to_string()))?;
        
        if !user.enabled {
            return Err(crate::error::AuthError::UserDisabled(username.to_string()));
        }
        
        if user.password_hash != password {
            return Err(crate::error::AuthError::InvalidCredentials);
        }
        
        let access_token = self.jwt_service.sign_token(&user.id, user.role)?;
        let refresh_token = self.jwt_service.sign_refresh_token(&user.id, user.role)?;
        
        Ok(LoginResult {
            user_id: user.id,
            username: user.username,
            role: user.role,
            access_token,
            refresh_token,
            expires_in: 86400, // 24 小时
        })
    }
    
    async fn logout(&self, _token: &str) -> AuthResult<()> {
        // 简化版本：生产环境应将令牌加入黑名单
        Ok(())
    }
    
    async fn verify_token(&self, token: &str) -> AuthResult<Claims> {
        self.jwt_service.verify_token(token)
    }
    
    async fn refresh_token(&self, refresh_token: &str) -> AuthResult<RefreshResult> {
        let (access_token, refresh_token) = self.jwt_service.refresh_token(refresh_token)?;
        
        Ok(RefreshResult {
            access_token,
            refresh_token,
            expires_in: 86400,
        })
    }
    
    async fn get_user_role(&self, user_id: &str) -> AuthResult<UserRole> {
        let user = self.users.read().unwrap()
            .iter()
            .find(|u| u.id == user_id)
            .cloned()
            .ok_or_else(|| crate::error::AuthError::UserNotFound(user_id.to_string()))?;
        
        Ok(UserRole::new(user.id, user.role))
    }
    
    async fn check_permission(&self, user_id: &str, permission: &str) -> AuthResult<bool> {
        let user_role = self.get_user_role(user_id).await?;
        let permission = crate::rbac::Permission::from_str(permission)
            .ok_or_else(|| crate::error::AuthError::InvalidToken(format!("Invalid permission: {}", permission)))?;
        
        Ok(user_role.has_permission(&permission))
    }

    async fn register(&self, username: &str, password: &str) -> AuthResult<LoginResult> {
        // 检查用户是否已存在
        if self.find_user(username).is_some() {
            return Err(crate::error::AuthError::InvalidCredentials);
        }

        // 添加新用户
        self.add_user(username, password, Role::User);

        // 登录并返回结果
        self.login(username, password).await
    }
}

/// 磁盘认证服务 - 持久化用户数据到 JSON 文件
pub struct DiskAuthService {
    /// 内存认证服务（核心逻辑）
    inner: MemoryAuthService,
    
    /// 用户数据文件路径
    users_file: PathBuf,
}

impl DiskAuthService {
    /// 创建新的磁盘认证服务
    /// 
    /// 从 `{storage_path}/users.json` 加载已有的用户数据（如果文件存在）。
    pub fn new(jwt_config: JwtConfig, secret: impl Into<String>, storage_path: &str) -> Self {
        let users_file = PathBuf::from(storage_path).join("users.json");
        let inner = MemoryAuthService::new(jwt_config, secret);
        
        // 尝试从文件加载用户数据
        if users_file.exists() {
            match Self::load_users_from_file(&users_file) {
                Ok(users) => {
                    inner.load_users(users);
                }
                Err(e) => {
                    eprintln!("[disk_auth] 加载用户数据失败: {}", e);
                }
            }
        }
        
        Self {
            inner,
            users_file,
        }
    }
    
    /// 从文件加载用户数据
    fn load_users_from_file(path: &PathBuf) -> Result<Vec<UserRecord>, Box<dyn std::error::Error>> {
        let data = std::fs::read_to_string(path)?;
        let users: Vec<UserRecord> = serde_json::from_str(&data)?;
        Ok(users)
    }
    
    /// 保存用户数据到文件
    fn save_users_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let users = self.inner.get_users();
        let json = serde_json::to_string_pretty(&users)?;
        
        // 确保父目录存在
        if let Some(parent) = self.users_file.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        std::fs::write(&self.users_file, json)?;
        Ok(())
    }
    
    /// 添加用户并持久化
    pub fn add_user(&self, username: &str, password: &str, role: Role) {
        self.inner.add_user(username, password, role);
        
        // 保存到磁盘
        if let Err(e) = self.save_users_to_file() {
            eprintln!("[disk_auth] 保存用户数据失败: {}", e);
        }
    }
    
    /// 获取用户数据文件路径
    pub fn users_file_path(&self) -> &PathBuf {
        &self.users_file
    }
}

#[async_trait]
impl AuthService for DiskAuthService {
    async fn login(&self, username: &str, password: &str) -> AuthResult<LoginResult> {
        self.inner.login(username, password).await
    }
    
    async fn logout(&self, token: &str) -> AuthResult<()> {
        self.inner.logout(token).await
    }
    
    async fn verify_token(&self, token: &str) -> AuthResult<Claims> {
        self.inner.verify_token(token).await
    }
    
    async fn refresh_token(&self, refresh_token: &str) -> AuthResult<RefreshResult> {
        self.inner.refresh_token(refresh_token).await
    }
    
    async fn get_user_role(&self, user_id: &str) -> AuthResult<UserRole> {
        self.inner.get_user_role(user_id).await
    }
    
    async fn check_permission(&self, user_id: &str, permission: &str) -> AuthResult<bool> {
        self.inner.check_permission(user_id, permission).await
    }

    async fn register(&self, username: &str, password: &str) -> AuthResult<LoginResult> {
        // 先注册
        let result = self.inner.register(username, password).await?;
        
        // 注册成功后保存到磁盘
        if let Err(e) = self.save_users_to_file() {
            eprintln!("[disk_auth] 注册后保存用户数据失败: {}", e);
        }
        
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_memory_auth_service() {
        let config = JwtConfig::default();
        let service = MemoryAuthService::new(config, "test-secret");
        
        // 添加用户
        service.add_user("admin", "password123", Role::Admin);
        service.add_user("user1", "password456", Role::User);
        
        // 测试登录
        let result = service.login("admin", "password123").await.unwrap();
        assert_eq!(result.role, Role::Admin);
        
        // 测试无效密码
        let result = service.login("admin", "wrongpassword").await;
        assert!(result.is_err());
        
        // 测试用户不存在
        let result = service.login("nonexistent", "password").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_token_verification() {
        let config = JwtConfig::default();
        let service = MemoryAuthService::new(config, "test-secret");
        
        service.add_user("user1", "password", Role::User);
        
        let login_result = service.login("user1", "password").await.unwrap();
        let claims = service.verify_token(&login_result.access_token).await.unwrap();
        
        assert_eq!(claims.sub, login_result.user_id);
    }
}
