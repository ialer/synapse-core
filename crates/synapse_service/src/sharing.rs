//! 共享授权模型
//!
//! 管理数据共享的请求、批准、拒绝和撤销流程。

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use data_core::{DataId, OwnerId, PermissionLevel};
use messaging_service::{Message, MessageType, MessagePriority};

/// 共享请求状态
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShareRequestStatus {
    Pending,
    Approved,
    Denied,
    Revoked,
    Expired,
}

/// 共享请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareRequest {
    pub id: Uuid,
    pub data_id: DataId,
    pub from_user: OwnerId,
    pub to_user: OwnerId,
    pub requested_level: PermissionLevel,
    pub message: Option<String>,
    pub status: ShareRequestStatus,
    pub created_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
}

/// 共享授权记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareGrant {
    pub id: Uuid,
    pub data_id: DataId,
    pub owner_id: OwnerId,
    pub grantee_id: OwnerId,
    pub level: PermissionLevel,
    pub granted_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub is_active: bool,
}

/// 共享管理器
pub struct ShareManager {
    requests: Vec<ShareRequest>,
    grants: Vec<ShareGrant>,
}

impl ShareManager {
    pub fn new() -> Self {
        Self {
            requests: Vec::new(),
            grants: Vec::new(),
        }
    }

    /// 创建共享请求
    pub fn request_share(
        &mut self,
        data_id: DataId,
        from: OwnerId,
        to: OwnerId,
        level: PermissionLevel,
        message: Option<String>,
    ) -> ShareRequest {
        let request = ShareRequest {
            id: Uuid::new_v4(),
            data_id,
            from_user: from,
            to_user: to,
            requested_level: level,
            message,
            status: ShareRequestStatus::Pending,
            created_at: Utc::now(),
            resolved_at: None,
        };
        self.requests.push(request.clone());
        request
    }

    /// 批准共享请求
    pub fn approve_request(
        &mut self,
        request_id: &Uuid,
        expires_at: Option<DateTime<Utc>>,
    ) -> Option<ShareGrant> {
        let request = self.requests.iter_mut().find(|r| r.id == *request_id)?;
        if request.status != ShareRequestStatus::Pending {
            return None;
        }

        let owner_id = request.to_user;
        let grantee_id = request.from_user;
        let data_id = request.data_id;
        let level = request.requested_level;

        request.status = ShareRequestStatus::Approved;
        request.resolved_at = Some(Utc::now());

        let grant = ShareGrant {
            id: Uuid::new_v4(),
            data_id,
            owner_id,
            grantee_id,
            level,
            granted_at: Utc::now(),
            expires_at,
            is_active: true,
        };
        self.grants.push(grant.clone());
        Some(grant)
    }

    /// 拒绝共享请求
    pub fn deny_request(&mut self, request_id: &Uuid) -> bool {
        if let Some(request) = self.requests.iter_mut().find(|r| r.id == *request_id) {
            if request.status == ShareRequestStatus::Pending {
                request.status = ShareRequestStatus::Denied;
                request.resolved_at = Some(Utc::now());
                return true;
            }
        }
        false
    }

    /// 撤销共享授权
    pub fn revoke_grant(&mut self, grant_id: &Uuid) -> bool {
        if let Some(grant) = self.grants.iter_mut().find(|g| g.id == *grant_id) {
            if grant.is_active {
                grant.is_active = false;
                return true;
            }
        }
        false
    }

    /// 创建共享请求并发送通知消息给接收者
    ///
    /// 创建一个 ShareRequest 并为接收者生成一个 Notification 类型的消息。
    /// 返回 (ShareRequest, Message) 元组。
    pub fn create_share_request_with_message(
        &mut self,
        data_id: DataId,
        from: OwnerId,
        to: OwnerId,
        level: PermissionLevel,
        message_text: Option<String>,
    ) -> (ShareRequest, Message) {
        let request = self.request_share(data_id, from, to, level, message_text.clone());

        let title = format!("共享请求 - {}", data_id);
        let content = message_text.unwrap_or_else(|| {
            format!(
                "用户 {} 请求以 {:?} 权限访问数据 {}",
                from, level, data_id
            )
        });

        let notification = Message::new(
            from.to_string(),
            to.to_string(),
            title,
            content,
        )
        .with_type(MessageType::Notification)
        .with_priority(MessagePriority::Normal);

        (request, notification)
    }

    /// 响应共享请求（批准或拒绝）并生成通知消息
    ///
    /// 如果 approved 为 true，则批准请求并创建审批通过的通知消息；
    /// 如果 approved 为 false，则拒绝请求并创建拒绝通知消息。
    /// 返回 (Option<ShareGrant>, Message) 元组。
    pub fn respond_to_request(
        &mut self,
        request_id: &Uuid,
        approved: bool,
        responder_id: OwnerId,
    ) -> (Option<ShareGrant>, Message) {
        if approved {
            let grant = self.approve_request(request_id, None);
            let (title, content) = if grant.is_some() {
                (
                    "共享请求已批准".to_string(),
                    "您的共享请求已被批准。您现在拥有对数据的访问权限。".to_string(),
                )
            } else {
                (
                    "共享请求处理失败".to_string(),
                    "共享请求处理失败，请求可能不存在或已被处理。".to_string(),
                )
            };

            let request = self
                .requests
                .iter()
                .find(|r| r.id == *request_id)
                .cloned();
            let recipient_id = request
                .map(|r| r.from_user.to_string())
                .unwrap_or_else(|| "unknown".to_string());

            let notification = Message::new(
                responder_id.to_string(),
                recipient_id,
                title,
                content,
            )
            .with_type(MessageType::Notification)
            .with_priority(MessagePriority::Normal);

            (grant, notification)
        } else {
            let success = self.deny_request(request_id);

            let (title, content) = if success {
                (
                    "共享请求已拒绝".to_string(),
                    "您的共享请求已被拒绝。".to_string(),
                )
            } else {
                (
                    "共享请求处理失败".to_string(),
                    "共享请求处理失败，请求可能不存在或已被处理。".to_string(),
                )
            };

            let request = self
                .requests
                .iter()
                .find(|r| r.id == *request_id)
                .cloned();
            let recipient_id = request
                .map(|r| r.from_user.to_string())
                .unwrap_or_else(|| "unknown".to_string());

            let notification = Message::new(
                responder_id.to_string(),
                recipient_id,
                title,
                content,
            )
            .with_type(MessageType::Notification)
            .with_priority(MessagePriority::Normal);

            (None, notification)
        }
    }

    /// 检查用户对数据的权限
    pub fn check_permission(
        &self,
        data_id: &DataId,
        user_id: &OwnerId,
        level: PermissionLevel,
    ) -> bool {
        self.grants.iter().any(|g| {
            g.data_id == *data_id
                && g.grantee_id == *user_id
                && g.is_active
                && !g.is_expired()
                && permission_meets(g.level, level)
        })
    }

    /// 获取用户的所有待处理共享请求（作为数据所有者收到的）
    pub fn get_pending_requests(&self, user_id: &OwnerId) -> Vec<&ShareRequest> {
        self.requests
            .iter()
            .filter(|r| r.to_user == *user_id && r.status == ShareRequestStatus::Pending)
            .collect()
    }

    /// 获取用户的所有共享授权
    pub fn get_grants_for_user(&self, user_id: &OwnerId) -> Vec<&ShareGrant> {
        self.grants
            .iter()
            .filter(|g| g.grantee_id == *user_id && g.is_active)
            .collect()
    }

    /// 获取数据的所有共享授权
    pub fn get_grants_for_data(&self, data_id: &DataId) -> Vec<&ShareGrant> {
        self.grants
            .iter()
            .filter(|g| g.data_id == *data_id && g.is_active)
            .collect()
    }
}

impl Default for ShareManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ShareGrant {
    /// 检查授权是否已过期
    pub fn is_expired(&self) -> bool {
        if let Some(exp) = self.expires_at {
            Utc::now() > exp
        } else {
            false
        }
    }

    /// 检查此授权是否满足所需权限级别
    pub fn level_meets(&self, required: PermissionLevel) -> bool {
        permission_meets(self.level, required)
    }
}

/// 检查权限级别是否满足所需级别
pub fn permission_meets(granted: PermissionLevel, required: PermissionLevel) -> bool {
    match (granted, required) {
        (PermissionLevel::Admin, _) => true,
        (PermissionLevel::Edit, PermissionLevel::Edit | PermissionLevel::View) => true,
        (PermissionLevel::View, PermissionLevel::View) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_request() {
        let mut mgr = ShareManager::new();
        let data_id = Uuid::new_v4();
        let from = Uuid::new_v4();
        let to = Uuid::new_v4();

        let req = mgr.request_share(
            data_id,
            from,
            to,
            PermissionLevel::View,
            Some("Please share".into()),
        );

        assert_eq!(req.status, ShareRequestStatus::Pending);
        assert_eq!(req.from_user, from);
        assert_eq!(req.to_user, to);
        assert_eq!(req.requested_level, PermissionLevel::View);
        assert_eq!(req.data_id, data_id);
        assert_eq!(req.message, Some("Please share".into()));
    }

    #[test]
    fn test_approve_request_creates_grant() {
        let mut mgr = ShareManager::new();
        let data_id = Uuid::new_v4();
        let owner = Uuid::new_v4();
        let requester = Uuid::new_v4();

        let req = mgr.request_share(
            data_id,
            requester,
            owner,
            PermissionLevel::Edit,
            None,
        );

        let grant = mgr.approve_request(&req.id, None).expect("approval should succeed");

        assert_eq!(grant.data_id, data_id);
        assert_eq!(grant.owner_id, owner);
        assert_eq!(grant.grantee_id, requester);
        assert_eq!(grant.level, PermissionLevel::Edit);
        assert!(grant.is_active);
        assert!(grant.expires_at.is_none());

        // Request should now be Approved
        let updated_req = mgr.requests.iter().find(|r| r.id == req.id).unwrap();
        assert_eq!(updated_req.status, ShareRequestStatus::Approved);
        assert!(updated_req.resolved_at.is_some());
    }

    #[test]
    fn test_approve_already_resolved_request_fails() {
        let mut mgr = ShareManager::new();
        let data_id = Uuid::new_v4();
        let owner = Uuid::new_v4();
        let requester = Uuid::new_v4();

        let req = mgr.request_share(data_id, requester, owner, PermissionLevel::View, None);
        mgr.approve_request(&req.id, None).unwrap();

        // Second approval should fail
        let result = mgr.approve_request(&req.id, None);
        assert!(result.is_none());
    }

    #[test]
    fn test_deny_request() {
        let mut mgr = ShareManager::new();
        let data_id = Uuid::new_v4();
        let owner = Uuid::new_v4();
        let requester = Uuid::new_v4();

        let req = mgr.request_share(data_id, requester, owner, PermissionLevel::View, None);
        let denied = mgr.deny_request(&req.id);
        assert!(denied);

        let updated_req = mgr.requests.iter().find(|r| r.id == req.id).unwrap();
        assert_eq!(updated_req.status, ShareRequestStatus::Denied);
        assert!(updated_req.resolved_at.is_some());
    }

    #[test]
    fn test_deny_already_resolved_fails() {
        let mut mgr = ShareManager::new();
        let data_id = Uuid::new_v4();
        let owner = Uuid::new_v4();
        let requester = Uuid::new_v4();

        let req = mgr.request_share(data_id, requester, owner, PermissionLevel::View, None);
        mgr.deny_request(&req.id);

        let second = mgr.deny_request(&req.id);
        assert!(!second);
    }

    #[test]
    fn test_revoke_grant() {
        let mut mgr = ShareManager::new();
        let data_id = Uuid::new_v4();
        let owner = Uuid::new_v4();
        let requester = Uuid::new_v4();

        let req = mgr.request_share(data_id, requester, owner, PermissionLevel::Edit, None);
        let grant = mgr.approve_request(&req.id, None).unwrap();

        // Should be active
        assert!(mgr.check_permission(&data_id, &requester, PermissionLevel::View));

        let revoked = mgr.revoke_grant(&grant.id);
        assert!(revoked);

        // Should no longer have permission
        assert!(!mgr.check_permission(&data_id, &requester, PermissionLevel::View));

        // Revoking again should fail
        let again = mgr.revoke_grant(&grant.id);
        assert!(!again);
    }

    #[test]
    fn test_permission_check_owner_has_admin() {
        let mut mgr = ShareManager::new();
        let data_id = Uuid::new_v4();
        let owner = Uuid::new_v4();
        let requester = Uuid::new_v4();

        let req = mgr.request_share(data_id, requester, owner, PermissionLevel::View, None);
        mgr.approve_request(&req.id, None).unwrap();

        // Grantee has View
        assert!(mgr.check_permission(&data_id, &requester, PermissionLevel::View));
        // Grantee does not have Edit or Admin
        assert!(!mgr.check_permission(&data_id, &requester, PermissionLevel::Edit));
        assert!(!mgr.check_permission(&data_id, &requester, PermissionLevel::Admin));
    }

    #[test]
    fn test_permission_check_admin_grantee_has_all() {
        let mut mgr = ShareManager::new();
        let data_id = Uuid::new_v4();
        let owner = Uuid::new_v4();
        let requester = Uuid::new_v4();

        let req = mgr.request_share(data_id, requester, owner, PermissionLevel::Admin, None);
        mgr.approve_request(&req.id, None).unwrap();

        assert!(mgr.check_permission(&data_id, &requester, PermissionLevel::View));
        assert!(mgr.check_permission(&data_id, &requester, PermissionLevel::Edit));
        assert!(mgr.check_permission(&data_id, &requester, PermissionLevel::Admin));
    }

    #[test]
    fn test_permission_check_edit_grantee_has_view_and_edit() {
        let mut mgr = ShareManager::new();
        let data_id = Uuid::new_v4();
        let owner = Uuid::new_v4();
        let requester = Uuid::new_v4();

        let req = mgr.request_share(data_id, requester, owner, PermissionLevel::Edit, None);
        mgr.approve_request(&req.id, None).unwrap();

        assert!(mgr.check_permission(&data_id, &requester, PermissionLevel::View));
        assert!(mgr.check_permission(&data_id, &requester, PermissionLevel::Edit));
        assert!(!mgr.check_permission(&data_id, &requester, PermissionLevel::Admin));
    }

    #[test]
    fn test_expiration_check() {
        let mut mgr = ShareManager::new();
        let data_id = Uuid::new_v4();
        let owner = Uuid::new_v4();
        let requester = Uuid::new_v4();

        // Grant that expired in the past
        let req = mgr.request_share(data_id, requester, owner, PermissionLevel::View, None);
        let past = Utc::now() - chrono::Duration::hours(1);
        mgr.approve_request(&req.id, Some(past)).unwrap();

        // Should be expired, so no permission
        assert!(!mgr.check_permission(&data_id, &requester, PermissionLevel::View));
    }

    #[test]
    fn test_active_not_expired_grant() {
        let mut mgr = ShareManager::new();
        let data_id = Uuid::new_v4();
        let owner = Uuid::new_v4();
        let requester = Uuid::new_v4();

        let req = mgr.request_share(data_id, requester, owner, PermissionLevel::Edit, None);
        let future = Utc::now() + chrono::Duration::hours(24);
        mgr.approve_request(&req.id, Some(future)).unwrap();

        assert!(mgr.check_permission(&data_id, &requester, PermissionLevel::View));
        assert!(mgr.check_permission(&data_id, &requester, PermissionLevel::Edit));
    }

    #[test]
    fn test_get_pending_requests() {
        let mut mgr = ShareManager::new();
        let data_id = Uuid::new_v4();
        let owner = Uuid::new_v4();
        let user_a = Uuid::new_v4();
        let user_b = Uuid::new_v4();

        mgr.request_share(data_id, user_a, owner, PermissionLevel::View, None);
        mgr.request_share(data_id, user_b, owner, PermissionLevel::Edit, None);

        let pending = mgr.get_pending_requests(&owner);
        assert_eq!(pending.len(), 2);

        // Approve one
        let id = pending[0].id;
        mgr.approve_request(&id, None);

        let pending_after = mgr.get_pending_requests(&owner);
        assert_eq!(pending_after.len(), 1);
    }

    #[test]
    fn test_get_grants_for_user() {
        let mut mgr = ShareManager::new();
        let user = Uuid::new_v4();
        let data1 = Uuid::new_v4();
        let data2 = Uuid::new_v4();
        let owner1 = Uuid::new_v4();
        let owner2 = Uuid::new_v4();

        let req1 = mgr.request_share(data1, user, owner1, PermissionLevel::View, None);
        mgr.approve_request(&req1.id, None).unwrap();

        let req2 = mgr.request_share(data2, user, owner2, PermissionLevel::Edit, None);
        mgr.approve_request(&req2.id, None).unwrap();

        let grants = mgr.get_grants_for_user(&user);
        assert_eq!(grants.len(), 2);
    }

    #[test]
    fn test_get_grants_for_data() {
        let mut mgr = ShareManager::new();
        let data_id = Uuid::new_v4();
        let owner = Uuid::new_v4();
        let user_a = Uuid::new_v4();
        let user_b = Uuid::new_v4();

        let req1 = mgr.request_share(data_id, user_a, owner, PermissionLevel::View, None);
        mgr.approve_request(&req1.id, None).unwrap();

        let req2 = mgr.request_share(data_id, user_b, owner, PermissionLevel::Edit, None);
        mgr.approve_request(&req2.id, None).unwrap();

        let grants = mgr.get_grants_for_data(&data_id);
        assert_eq!(grants.len(), 2);
    }

    #[test]
    fn test_multiple_users_sharing_same_data() {
        let mut mgr = ShareManager::new();
        let data_id = Uuid::new_v4();
        let owner = Uuid::new_v4();
        let viewer = Uuid::new_v4();
        let editor = Uuid::new_v4();
        let admin = Uuid::new_v4();

        let req_v = mgr.request_share(data_id, viewer, owner, PermissionLevel::View, None);
        mgr.approve_request(&req_v.id, None).unwrap();

        let req_e = mgr.request_share(data_id, editor, owner, PermissionLevel::Edit, None);
        mgr.approve_request(&req_e.id, None).unwrap();

        let req_a = mgr.request_share(data_id, admin, owner, PermissionLevel::Admin, None);
        mgr.approve_request(&req_a.id, None).unwrap();

        // Viewer can only view
        assert!(mgr.check_permission(&data_id, &viewer, PermissionLevel::View));
        assert!(!mgr.check_permission(&data_id, &viewer, PermissionLevel::Edit));

        // Editor can view and edit
        assert!(mgr.check_permission(&data_id, &editor, PermissionLevel::View));
        assert!(mgr.check_permission(&data_id, &editor, PermissionLevel::Edit));
        assert!(!mgr.check_permission(&data_id, &editor, PermissionLevel::Admin));

        // Admin has all
        assert!(mgr.check_permission(&data_id, &admin, PermissionLevel::View));
        assert!(mgr.check_permission(&data_id, &admin, PermissionLevel::Edit));
        assert!(mgr.check_permission(&data_id, &admin, PermissionLevel::Admin));

        // All three active grants
        let grants = mgr.get_grants_for_data(&data_id);
        assert_eq!(grants.len(), 3);
    }

    #[test]
    fn test_revoked_grant_not_listed() {
        let mut mgr = ShareManager::new();
        let data_id = Uuid::new_v4();
        let owner = Uuid::new_v4();
        let user = Uuid::new_v4();

        let req = mgr.request_share(data_id, user, owner, PermissionLevel::View, None);
        let grant = mgr.approve_request(&req.id, None).unwrap();
        mgr.revoke_grant(&grant.id);

        // Active grants for user should be empty
        let active = mgr.get_grants_for_user(&user);
        assert_eq!(active.len(), 0);

        // Active grants for data should be empty
        let data_grants = mgr.get_grants_for_data(&data_id);
        assert_eq!(data_grants.len(), 0);
    }

    #[test]
    fn test_approve_nonexistent_request() {
        let mut mgr = ShareManager::new();
        let fake_id = Uuid::new_v4();
        assert!(mgr.approve_request(&fake_id, None).is_none());
    }

    #[test]
    fn test_deny_nonexistent_request() {
        let mut mgr = ShareManager::new();
        let fake_id = Uuid::new_v4();
        assert!(!mgr.deny_request(&fake_id));
    }

    #[test]
    fn test_create_share_request_with_message() {
        let mut mgr = ShareManager::new();
        let data_id = Uuid::new_v4();
        let from = Uuid::new_v4();
        let to = Uuid::new_v4();

        let (request, notification) = mgr.create_share_request_with_message(
            data_id,
            from,
            to,
            PermissionLevel::View,
            Some("请共享数据".to_string()),
        );

        // Verify the request
        assert_eq!(request.status, ShareRequestStatus::Pending);
        assert_eq!(request.from_user, from);
        assert_eq!(request.to_user, to);
        assert_eq!(request.data_id, data_id);
        assert_eq!(request.requested_level, PermissionLevel::View);

        // Verify the notification message
        assert_eq!(notification.sender_id, from.to_string());
        assert_eq!(notification.recipient_id, to.to_string());
        assert!(notification.title.contains(&data_id.to_string()));
        assert_eq!(notification.content, "请共享数据");
        assert!(matches!(notification.message_type, MessageType::Notification));
        assert!(matches!(
            notification.priority,
            MessagePriority::Normal
        ));
        assert!(!notification.is_read());
    }

    #[test]
    fn test_create_share_request_with_message_default_content() {
        let mut mgr = ShareManager::new();
        let data_id = Uuid::new_v4();
        let from = Uuid::new_v4();
        let to = Uuid::new_v4();

        let (_request, notification) = mgr.create_share_request_with_message(
            data_id,
            from,
            to,
            PermissionLevel::Edit,
            None,
        );

        // When no custom message, default content should contain from user and level
        assert!(notification.content.contains(&from.to_string()));
        assert!(notification.content.contains("Edit"));
    }

    #[test]
    fn test_respond_to_request_approve() {
        let mut mgr = ShareManager::new();
        let data_id = Uuid::new_v4();
        let requester = Uuid::new_v4();
        let owner = Uuid::new_v4();

        let req = mgr.request_share(
            data_id,
            requester,
            owner,
            PermissionLevel::View,
            None,
        );

        let (grant, notification) = mgr.respond_to_request(&req.id, true, owner);

        // Verify the grant was created
        let grant = grant.expect("grant should be created");
        assert_eq!(grant.data_id, data_id);
        assert_eq!(grant.owner_id, owner);
        assert_eq!(grant.grantee_id, requester);
        assert!(grant.is_active);

        // Verify the approval notification
        assert_eq!(notification.sender_id, owner.to_string());
        assert_eq!(notification.recipient_id, requester.to_string());
        assert!(notification.title.contains("批准"));
        assert!(notification.content.contains("批准"));
        assert!(matches!(notification.message_type, MessageType::Notification));
    }

    #[test]
    fn test_respond_to_request_deny() {
        let mut mgr = ShareManager::new();
        let data_id = Uuid::new_v4();
        let requester = Uuid::new_v4();
        let owner = Uuid::new_v4();

        let req = mgr.request_share(
            data_id,
            requester,
            owner,
            PermissionLevel::View,
            None,
        );

        let (grant, notification) = mgr.respond_to_request(&req.id, false, owner);

        // Verify no grant was created
        assert!(grant.is_none());

        // Verify the request was denied
        let updated_req = mgr.requests.iter().find(|r| r.id == req.id).unwrap();
        assert_eq!(updated_req.status, ShareRequestStatus::Denied);

        // Verify the denial notification
        assert_eq!(notification.sender_id, owner.to_string());
        assert_eq!(notification.recipient_id, requester.to_string());
        assert!(notification.title.contains("拒绝"));
        assert!(notification.content.contains("拒绝"));
        assert!(matches!(notification.message_type, MessageType::Notification));
    }

    #[test]
    fn test_respond_to_request_approve_nonexistent() {
        let mut mgr = ShareManager::new();
        let owner = Uuid::new_v4();
        let fake_id = Uuid::new_v4();

        let (grant, notification) = mgr.respond_to_request(&fake_id, true, owner);

        assert!(grant.is_none());
        assert!(notification.title.contains("失败"));
    }

    #[test]
    fn test_respond_to_request_deny_nonexistent() {
        let mut mgr = ShareManager::new();
        let owner = Uuid::new_v4();
        let fake_id = Uuid::new_v4();

        let (grant, notification) = mgr.respond_to_request(&fake_id, false, owner);

        assert!(grant.is_none());
        assert!(notification.title.contains("失败"));
    }

    #[test]
    fn test_full_sharing_flow_with_messages() {
        let mut mgr = ShareManager::new();
        let data_id = Uuid::new_v4();
        let requester = Uuid::new_v4();
        let owner = Uuid::new_v4();

        // Step 1: Requester creates share request with notification
        let (req, request_msg) = mgr.create_share_request_with_message(
            data_id,
            requester,
            owner,
            PermissionLevel::Edit,
            Some("我需要编辑权限".to_string()),
        );

        assert_eq!(req.status, ShareRequestStatus::Pending);
        assert_eq!(request_msg.sender_id, requester.to_string());
        assert_eq!(request_msg.recipient_id, owner.to_string());
        assert!(request_msg.content.contains("我需要编辑权限"));

        // Step 2: Owner approves the request
        let (grant, approval_msg) = mgr.respond_to_request(&req.id, true, owner);

        let grant = grant.expect("grant should be created");
        assert!(grant.is_active);
        assert_eq!(grant.level, PermissionLevel::Edit);
        assert_eq!(approval_msg.sender_id, owner.to_string());
        assert_eq!(approval_msg.recipient_id, requester.to_string());

        // Step 3: Verify requester now has permission
        assert!(mgr.check_permission(&data_id, &requester, PermissionLevel::View));
        assert!(mgr.check_permission(&data_id, &requester, PermissionLevel::Edit));

        // Step 4: Owner revokes the grant
        let revoked = mgr.revoke_grant(&grant.id);
        assert!(revoked);
        assert!(!mgr.check_permission(&data_id, &requester, PermissionLevel::View));
    }

    #[test]
    fn test_full_deny_flow_with_messages() {
        let mut mgr = ShareManager::new();
        let data_id = Uuid::new_v4();
        let requester = Uuid::new_v4();
        let owner = Uuid::new_v4();

        // Step 1: Requester creates share request
        let (req, _request_msg) = mgr.create_share_request_with_message(
            data_id,
            requester,
            owner,
            PermissionLevel::Admin,
            None,
        );

        assert_eq!(req.status, ShareRequestStatus::Pending);

        // Step 2: Owner denies the request
        let (grant, denial_msg) = mgr.respond_to_request(&req.id, false, owner);

        assert!(grant.is_none());
        assert_eq!(denial_msg.sender_id, owner.to_string());
        assert_eq!(denial_msg.recipient_id, requester.to_string());
        assert!(denial_msg.title.contains("拒绝"));

        // Step 3: Verify requester has no permission
        assert!(!mgr.check_permission(&data_id, &requester, PermissionLevel::View));
        assert!(!mgr.check_permission(&data_id, &requester, PermissionLevel::Admin));

        // Step 4: Verify request is in Denied state
        let updated = mgr.requests.iter().find(|r| r.id == req.id).unwrap();
        assert_eq!(updated.status, ShareRequestStatus::Denied);
    }
}
