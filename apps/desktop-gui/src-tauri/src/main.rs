// Tauri 全平台主入口 (Desktop + Mobile)

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::State;

use synapse_service::SynapseApp;
use data_core::DataType;

/// 应用状态
struct AppState {
    app: tokio::sync::Mutex<SynapseApp>,
}

/// 登录命令
#[tauri::command]
async fn login(state: State<'_, AppState>, username: String, password: String) -> Result<String, String> {
    let app = state.app.lock().await;
    app.login(&username, &password)
        .await
        .map_err(|e| e.to_string())
}

/// 存储数据命令
#[tauri::command]
async fn store_data(
    state: State<'_, AppState>,
    token: String,
    data_type: String,
    content: String,
    tags: Vec<String>,
) -> Result<String, String> {
    let mut app = state.app.lock().await;

    let dt = match data_type.as_str() {
        "credential" => DataType::Credential,
        "config" => DataType::Config,
        "file" => DataType::File,
        "contact" => DataType::Contact,
        _ => DataType::Generic,
    };

    let entity = app.secure_store(&token, dt, content.into_bytes(), tags)
        .await
        .map_err(|e| e.to_string())?;

    Ok(entity.id.to_string())
}

/// 获取数据命令
#[tauri::command]
async fn get_data(state: State<'_, AppState>, token: String, id: String) -> Result<String, String> {
    let app = state.app.lock().await;

    let entity = app.secure_get(&token, &id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(String::from_utf8_lossy(&entity.encrypted_content).to_string())
}

/// 搜索数据命令
#[tauri::command]
async fn search_data(state: State<'_, AppState>, query: String, limit: usize) -> Result<Vec<SearchResult>, String> {
    let app = state.app.lock().await;

    let results = app.search(&query, limit);
    let search_results: Vec<SearchResult> = results.into_iter().map(|r| {
        SearchResult {
            id: r.id,
            content: r.content,
            metadata: r.metadata,
        }
    }).collect();

    Ok(search_results)
}

/// 删除数据命令
#[tauri::command]
async fn delete_data(state: State<'_, AppState>, token: String, id: String) -> Result<bool, String> {
    let mut app = state.app.lock().await;

    app.secure_delete(&token, &id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(true)
}

/// 获取统计信息命令
#[tauri::command]
async fn get_stats(state: State<'_, AppState>) -> Result<StatsInfo, String> {
    let app = state.app.lock().await;
    let stats = app.stats();

    Ok(StatsInfo {
        data_count: stats.data_count,
        index_count: stats.index_count,
        message_count: stats.message_count,
    })
}

/// 发送消息命令
#[tauri::command]
async fn send_message(
    state: State<'_, AppState>,
    token: String,
    recipient_id: String,
    title: String,
    content: String,
) -> Result<bool, String> {
    let mut app = state.app.lock().await;

    app.send_message(&token, &recipient_id, &title, &content)
        .map_err(|e| e.to_string())?;

    Ok(true)
}

/// 获取用户消息命令
#[tauri::command]
async fn get_messages(state: State<'_, AppState>, user_id: String, limit: usize) -> Result<Vec<MessageInfo>, String> {
    let app = state.app.lock().await;

    let messages = app.get_messages(&user_id, limit);
    let message_infos: Vec<MessageInfo> = messages.into_iter().map(|m| {
        MessageInfo {
            id: m.id.to_string(),
            from: m.sender_id.clone(),
            to: m.recipient_id.clone(),
            title: m.title.clone(),
            content: m.content.clone(),
            timestamp: m.sent_at.to_string(),
        }
    }).collect();

    Ok(message_infos)
}

/// 列出所有数据命令
#[tauri::command]
async fn list_data(state: State<'_, AppState>, token: String) -> Result<Vec<DataItemInfo>, String> {
    let app = state.app.lock().await;

    // 验证 token
    app.verify_token(&token).await.map_err(|e| e.to_string())?;

    let items: Vec<DataItemInfo> = app.data_store.values().map(|entity| {
        DataItemInfo {
            id: entity.id.to_string(),
            data_type: entity.data_type.as_str().to_string(),
            tags: entity.tags.clone(),
            created_at: entity.created_at.to_rfc3339(),
        }
    }).collect();

    Ok(items)
}

/// 更新数据命令
#[tauri::command]
async fn update_data(
    state: State<'_, AppState>,
    token: String,
    id: String,
    content: String,
    tags: Vec<String>,
) -> Result<bool, String> {
    let mut app = state.app.lock().await;

    app.secure_update(&token, &id, content.into_bytes(), tags)
        .await
        .map_err(|e| e.to_string())
}

/// 获取存储信息命令
#[tauri::command]
async fn get_storage_info(state: State<'_, AppState>) -> Result<StorageInfo, String> {
    let _app = state.app.lock().await;

    // 当前使用 LocalBackend（在 new_local 中创建）
    Ok(StorageInfo {
        backend_type: "local".to_string(),
        is_configured: true,
    })
}

/// 注册用户命令
#[tauri::command]
async fn register_user(state: State<'_, AppState>, username: String, password: String) -> Result<String, String> {
    let mut app = state.app.lock().await;

    app.register(&username, &password)
        .await
        .map_err(|e| e.to_string())
}

/// 搜索结果
#[derive(serde::Serialize)]
struct SearchResult {
    id: String,
    content: String,
    metadata: std::collections::HashMap<String, String>,
}

/// 统计信息
#[derive(serde::Serialize)]
struct StatsInfo {
    data_count: usize,
    index_count: usize,
    message_count: usize,
}

/// 消息信息
#[derive(serde::Serialize)]
struct MessageInfo {
    id: String,
    from: String,
    to: String,
    title: String,
    content: String,
    timestamp: String,
}

/// 数据条目信息（用于列表展示）
#[derive(serde::Serialize)]
struct DataItemInfo {
    id: String,
    data_type: String,
    tags: Vec<String>,
    created_at: String,
}

/// 存储信息
#[derive(serde::Serialize)]
struct StorageInfo {
    backend_type: String,
    is_configured: bool,
}

fn main() {
    // Create app first
    let mut app = SynapseApp::new_local("/tmp/synapse-data")
        .expect("Failed to create SynapseApp");

    // Initialize from disk: load data_store and indexer
    let rt = tokio::runtime::Runtime::new()
        .expect("Failed to create Tokio runtime for init");
    rt.block_on(app.init())
        .expect("Failed to initialize SynapseApp from disk");

    tauri::Builder::default()
        .manage(AppState {
            app: tokio::sync::Mutex::new(app),
        })
        .invoke_handler(tauri::generate_handler![
            login,
            store_data,
            get_data,
            search_data,
            delete_data,
            get_stats,
            send_message,
            get_messages,
            list_data,
            update_data,
            get_storage_info,
            register_user,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
