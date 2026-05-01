//! RBAC 角色与权限模块
//! 
//! 定义基于角色的访问控制模型。

use serde::{Deserialize, Serialize};
use std::fmt;

/// 角色枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Role {
    /// 管理员 - 拥有所有权限
    Admin,
    /// 普通用户 - 拥有基础权限
    User,
    /// 访客 - 只有只读权限
    Guest,
}

impl Role {
    /// 获取角色的字符串表示
    pub fn as_str(&self) -> &'static str {
        match self {
            Role::Admin => "admin",
            Role::User => "user",
            Role::Guest => "guest",
        }
    }
    
    /// 从字符串解析角色
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "admin" => Some(Role::Admin),
            "user" => Some(Role::User),
            "guest" => Some(Role::Guest),
            _ => None,
        }
    }
    
    /// 获取角色的权限列表
    pub fn permissions(&self) -> Vec<Permission> {
        match self {
            Role::Admin => vec![
                Permission::Read,
                Permission::Write,
                Permission::Delete,
                Permission::Admin,
            ],
            Role::User => vec![
                Permission::Read,
                Permission::Write,
            ],
            Role::Guest => vec![
                Permission::Read,
            ],
        }
    }
    
    /// 获取角色的层级（数字越大权限越高）
    pub fn level(&self) -> u8 {
        match self {
            Role::Admin => 100,
            Role::User => 50,
            Role::Guest => 10,
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for Role {
    fn default() -> Self {
        Role::Guest
    }
}

/// 权限枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Permission {
    /// 读取权限
    Read,
    /// 写入权限
    Write,
    /// 删除权限
    Delete,
    /// 管理权限
    Admin,
}

impl Permission {
    /// 获取权限的字符串表示
    pub fn as_str(&self) -> &'static str {
        match self {
            Permission::Read => "read",
            Permission::Write => "write",
            Permission::Delete => "delete",
            Permission::Admin => "admin",
        }
    }
    
    /// 从字符串解析权限
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "read" => Some(Permission::Read),
            "write" => Some(Permission::Write),
            "delete" => Some(Permission::Delete),
            "admin" => Some(Permission::Admin),
            _ => None,
        }
    }
    
    /// 获取权限的层级
    pub fn level(&self) -> u8 {
        match self {
            Permission::Read => 10,
            Permission::Write => 50,
            Permission::Delete => 80,
            Permission::Admin => 100,
        }
    }
}

impl fmt::Display for Permission {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// 用户角色分配
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRole {
    /// 用户 ID
    pub user_id: String,
    
    /// 角色
    pub role: Role,
    
    /// 额外权限（超出角色默认权限）
    pub extra_permissions: Vec<Permission>,
    
    /// 分配时间
    pub assigned_at: chrono::DateTime<chrono::Utc>,
}

impl UserRole {
    /// 创建新的用户角色分配
    pub fn new(user_id: impl Into<String>, role: Role) -> Self {
        Self {
            user_id: user_id.into(),
            role,
            extra_permissions: Vec::new(),
            assigned_at: chrono::Utc::now(),
        }
    }
    
    /// 添加额外权限
    pub fn with_permission(mut self, permission: Permission) -> Self {
        if !self.extra_permissions.contains(&permission) {
            self.extra_permissions.push(permission);
        }
        self
    }
    
    /// 检查是否拥有指定权限
    pub fn has_permission(&self, permission: &Permission) -> bool {
        // 检查角色默认权限
        if self.role.permissions().contains(permission) {
            return true;
        }
        
        // 检查额外权限
        self.extra_permissions.contains(permission)
    }
    
    /// 获取所有权限（角色权限 + 额外权限）
    pub fn all_permissions(&self) -> Vec<Permission> {
        let mut permissions = self.role.permissions();
        for perm in &self.extra_permissions {
            if !permissions.contains(perm) {
                permissions.push(*perm);
            }
        }
        permissions
    }
}

/// 权限检查器
pub struct PermissionChecker;

impl PermissionChecker {
    /// 检查用户是否拥有所需权限
    pub fn check_permission(user_role: &UserRole, required: &Permission) -> bool {
        user_role.has_permission(required)
    }
    
    /// 检查用户是否拥有所需角色
    pub fn check_role(user_role: &UserRole, required: &Role) -> bool {
        user_role.role.level() >= required.level()
    }
    
    /// 检查用户是否拥有所有所需权限
    pub fn check_permissions(user_role: &UserRole, required: &[Permission]) -> bool {
        required.iter().all(|p| user_role.has_permission(p))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_permissions() {
        assert_eq!(Role::Admin.permissions().len(), 4);
        assert_eq!(Role::User.permissions().len(), 2);
        assert_eq!(Role::Guest.permissions().len(), 1);
    }

    #[test]
    fn test_role_level() {
        assert!(Role::Admin.level() > Role::User.level());
        assert!(Role::User.level() > Role::Guest.level());
    }

    #[test]
    fn test_permission_level() {
        assert!(Permission::Admin.level() > Permission::Delete.level());
        assert!(Permission::Delete.level() > Permission::Write.level());
        assert!(Permission::Write.level() > Permission::Read.level());
    }

    #[test]
    fn test_user_role() {
        let user_role = UserRole::new("user1", Role::User);
        assert!(user_role.has_permission(&Permission::Read));
        assert!(user_role.has_permission(&Permission::Write));
        assert!(!user_role.has_permission(&Permission::Delete));
    }

    #[test]
    fn test_user_role_with_extra_permission() {
        let user_role = UserRole::new("user1", Role::User)
            .with_permission(Permission::Delete);
        assert!(user_role.has_permission(&Permission::Delete));
    }

    #[test]
    fn test_permission_checker() {
        let user_role = UserRole::new("user1", Role::User);
        assert!(PermissionChecker::check_permission(&user_role, &Permission::Read));
        assert!(!PermissionChecker::check_permission(&user_role, &Permission::Delete));
        assert!(PermissionChecker::check_role(&user_role, &Role::Guest));
        assert!(!PermissionChecker::check_role(&user_role, &Role::Admin));
    }
}
