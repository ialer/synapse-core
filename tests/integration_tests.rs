//! SynapseCore 集成测试
//!
//! 端到端测试覆盖：注册→登录→存储→检索→共享→删除→搜索

use tempfile::TempDir;

/// 辅助函数：创建临时应用实例并初始化
async fn create_test_app() -> (synapse_service::SynapseApp, TempDir) {
    let temp_dir = TempDir::new().unwrap();
    let mut app = synapse_service::SynapseApp::new_local(temp_dir.path().to_str().unwrap())
        .await
        .unwrap();
    app.init().await.unwrap();
    (app, temp_dir)
}

// ===========================================================================
// 核心 CRUD 流程
// ===========================================================================

#[tokio::test]
async fn test_e2e_register_login_store_get() {
    let (mut app, _dir) = create_test_app().await;

    // 注册
    let token = app.register("alice", "pass123").await.unwrap();
    assert!(!token.is_empty(), "注册应返回有效 token");

    // 登录
    let token2 = app.login("alice", "pass123").await.unwrap();
    assert!(!token2.is_empty(), "登录应返回有效 token");

    // 登录错误密码
    let bad = app.login("alice", "wrong").await;
    assert!(bad.is_err(), "错误密码应返回错误");

    // 存储数据
    let entity = app
        .secure_store(
            &token,
            data_core::DataType::Credential,
            b"my secret".to_vec(),
            vec!["github".to_string()],
        )
        .await
        .unwrap();
    let id = entity.id.to_string();
    assert!(!id.is_empty(), "存储应返回有效 ID");

    // 获取数据 (加密)
    let got = app.secure_get(&token, &id).await.unwrap();
    assert_eq!(got.id.to_string(), id);

    // 获取解密数据
    let (got_entity, decrypted) = app.secure_get_decrypted(&token, &id).await.unwrap();
    assert_eq!(decrypted, b"my secret");
    assert_eq!(got_entity.id.to_string(), id);

    // 统计
    let stats = app.stats();
    assert_eq!(stats.data_count, 1);
}

#[tokio::test]
async fn test_e2e_update_data() {
    let (mut app, _dir) = create_test_app().await;
    let token = app.register("bob", "pass123").await.unwrap();

    // 存储
    let entity = app
        .secure_store(&token, data_core::DataType::Config, b"v1".to_vec(), vec![])
        .await
        .unwrap();
    let id = entity.id.to_string();

    // 更新
    app.secure_update(&token, &id, b"v2".to_vec(), vec!["updated".to_string()])
        .await
        .unwrap();

    // 验证更新
    let (_, decrypted) = app.secure_get_decrypted(&token, &id).await.unwrap();
    assert_eq!(decrypted, b"v2");
}

#[tokio::test]
async fn test_e2e_delete_data() {
    let (mut app, _dir) = create_test_app().await;
    let token = app.register("carol", "pass123").await.unwrap();

    // 存储
    let entity = app
        .secure_store(
            &token,
            data_core::DataType::Generic,
            b"to delete".to_vec(),
            vec![],
        )
        .await
        .unwrap();
    let id = entity.id.to_string();
    assert_eq!(app.get_data_count(&token).await.unwrap(), 1);

    // 删除
    app.secure_delete(&token, &id).await.unwrap();
    assert_eq!(app.get_data_count(&token).await.unwrap(), 0);

    // 删除后不应能获取
    let result = app.secure_get(&token, &id).await;
    assert!(result.is_err(), "删除后不应能获取数据");
}

// ===========================================================================
// 权限与隔离
// ===========================================================================

#[tokio::test]
async fn test_e2e_user_isolation() {
    let (mut app, _dir) = create_test_app().await;

    let token_a = app.register("user_a", "pass1").await.unwrap();
    let token_b = app.register("user_b", "pass2").await.unwrap();

    // A 存储数据
    let entity = app
        .secure_store(
            &token_a,
            data_core::DataType::Credential,
            b"a_secret".to_vec(),
            vec![],
        )
        .await
        .unwrap();
    let id = entity.id.to_string();

    // B 不应能读取 A 的数据
    let result = app.secure_get(&token_b, &id).await;
    assert!(result.is_err(), "用户 B 不应能读取用户 A 的数据");

    // B 不应能删除 A 的数据
    let result = app.secure_delete(&token_b, &id).await;
    assert!(result.is_err(), "用户 B 不应能删除用户 A 的数据");
}

// ===========================================================================
// 搜索功能
// ===========================================================================

#[tokio::test]
async fn test_e2e_search_workflow() {
    let (mut app, _dir) = create_test_app().await;
    let token = app.register("searcher", "pass123").await.unwrap();

    // 存储多条数据
    app.secure_store(
        &token,
        data_core::DataType::Credential,
        b"github_token_abc".to_vec(),
        vec!["github".to_string(), "token".to_string()],
    )
    .await
    .unwrap();

    app.secure_store(
        &token,
        data_core::DataType::Credential,
        b"aws_access_key_xyz".to_vec(),
        vec!["aws".to_string(), "cloud".to_string()],
    )
    .await
    .unwrap();

    app.secure_store(
        &token,
        data_core::DataType::Config,
        b"database_config".to_vec(),
        vec!["database".to_string()],
    )
    .await
    .unwrap();

    // 搜索
    let results = app.search(&token, "github", 10).await.unwrap();
    assert_eq!(results.len(), 1, "应找到 1 条 github 相关数据");

    let results = app.search(&token, "token", 10).await.unwrap();
    assert!(results.len() >= 1, "应找到 token 相关数据");

    // 标签搜索
    let results = app.search_by_tag("cloud", 10);
    assert_eq!(results.len(), 1, "应找到 1 条 cloud 标签数据");

    // 统计
    let stats = app.stats();
    assert_eq!(stats.data_count, 3);
}

// ===========================================================================
// 数据类型
// ===========================================================================

#[tokio::test]
async fn test_e2e_all_data_types() {
    let (mut app, _dir) = create_test_app().await;
    let token = app.register("types_user", "pass123").await.unwrap();

    let types = vec![
        data_core::DataType::Credential,
        data_core::DataType::Config,
        data_core::DataType::File,
        data_core::DataType::Contact,
        data_core::DataType::Generic,
    ];

    for dt in types {
        let entity = app
            .secure_store(&token, dt.clone(), b"test_data".to_vec(), vec![])
            .await
            .unwrap();
        assert!(!entity.id.to_string().is_empty());
    }

    assert_eq!(app.get_data_count(&token).await.unwrap(), 5);
}

// ===========================================================================
// 数据列表与分页
// ===========================================================================

#[tokio::test]
async fn test_e2e_list_data() {
    let (mut app, _dir) = create_test_app().await;
    let token = app.register("lister", "pass123").await.unwrap();

    // 存储 5 条数据
    for i in 0..5 {
        app.secure_store(
            &token,
            data_core::DataType::Generic,
            format!("data_{}", i).into_bytes(),
            vec![],
        )
        .await
        .unwrap();
    }

    // 列出所有
    let all = app.list_all_data(&token).await.unwrap();
    assert_eq!(all.len(), 5, "应列出 5 条数据");

    // 统计
    assert_eq!(app.get_data_count(&token).await.unwrap(), 5);
}

// ===========================================================================
// 消息服务
// ===========================================================================

#[tokio::test]
async fn test_e2e_messaging() {
    let (mut app, _dir) = create_test_app().await;

    // 注册用户获取真实 token
    let token_a = app.register("alice", "pass1").await.unwrap();
    let token_b = app.register("bob", "pass2").await.unwrap();

    // 发送消息（需要认证）
    app.send_message(&token_a, "bob", "Greeting", "Hello Bob!")
        .await
        .unwrap();
    app.send_message(&token_b, "alice", "Reply", "Hi Alice!")
        .await
        .unwrap();

    // 获取消息
    let messages = app.get_messages("bob", 10);
    assert_eq!(messages.len(), 1, "Bob 应收到 1 条消息");
    assert_eq!(messages[0].content, "Hello Bob!");
}

// ===========================================================================
// 数据持久化
// ===========================================================================

#[tokio::test]
async fn test_e2e_persistence() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().to_str().unwrap();

    // 第一个实例：存储数据
    {
        let mut app = synapse_service::SynapseApp::new_local(path).await.unwrap();
        app.init().await.unwrap();
        let token = app.register("persist_user", "pass123").await.unwrap();
        app.secure_store(
            &token,
            data_core::DataType::Credential,
            b"persistent_data".to_vec(),
            vec!["persistent".to_string()],
        )
        .await
        .unwrap();
    }

    // 第二个实例：验证数据持久化
    {
        let mut app = synapse_service::SynapseApp::new_local(path).await.unwrap();
        app.init().await.unwrap();

        // 数据应已加载 - 先登录获取 token
        let token = app.login("persist_user", "pass123").await.unwrap();
        assert_eq!(app.get_data_count(&token).await.unwrap(), 1);

        // search_by_tag is still public (no auth needed for tag search)
        let results = app.search_by_tag("persistent", 10);
        assert_eq!(results.len(), 1);
    }
}

// ===========================================================================
// 安全性验证
// ===========================================================================

#[tokio::test]
async fn test_e2e_encryption_roundtrip() {
    let (mut app, _dir) = create_test_app().await;
    let token = app.register("crypto_user", "pass123").await.unwrap();

    let original = b"The quick brown fox jumps over the lazy dog. 1234567890 !@#$%^&*()";
    let entity = app
        .secure_store(&token, data_core::DataType::File, original.to_vec(), vec![])
        .await
        .unwrap();
    let id = entity.id.to_string();

    // 存储的内容应该是加密的
    assert_ne!(entity.encrypted_content, original, "存储的内容应已加密");

    // 解密后应与原文一致
    let (_, decrypted) = app.secure_get_decrypted(&token, &id).await.unwrap();
    assert_eq!(decrypted, original, "解密后应与原文一致");
}
