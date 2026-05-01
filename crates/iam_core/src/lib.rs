//! # IAM Core - 身份认证模块
//! 
//! 提供基于角色的访问控制（RBAC）与 JWT 认证功能。
//! 
//! ## 核心功能
//! 
//! - AuthService: 认证服务 Trait
//! - RBAC: 角色与权限管理
//! - JWT: 令牌签发与校验

pub mod auth;
pub mod rbac;
pub mod jwt;
pub mod error;

pub use auth::{AuthService, MemoryAuthService, LoginResult, RefreshResult};
pub use rbac::{Role, Permission, UserRole, PermissionChecker};
pub use jwt::{Claims, JwtConfig, JwtService};
pub use error::{AuthError, AuthResult};
