// Tauri 桌面端主入口

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use tauri::State;

use synapse_service::SynapseApp;
use data_core::DataType;

/// 应用状态
struct AppState {
    app: Mutex<SynapseApp>,
}

/// 登录命令
#[tauri::command]
async fn login(state: State<'_, AppState>, username: String, password: String) -> Result<String, String> {
    let app = state.app.lock().map_err(|e| e.to_string())?;
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
    let mut app = state.app.lock().map_err(|e| e.to_string())?;
    
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
    let app = state.app.lock().map_err(|e| e.to_string())?;
    
    let entity = app.secure_get(&token, &id)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(String::from_utf8_lossy(&entity.encrypted_content).to_string())
}

/// 搜索数据命令
#[tauri::command]
fn search_data(state: State<'_, AppState>, query: String, limit: usize) -> Result<Vec<SearchResult>, String> {
    let app = state.app.lock().map_err(|e| e.to_string())?;
    
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
    let mut app = state.app.lock().map_err(|e| e.to_string())?;
    
    app.secure_delete(&token, &id)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(true)
}

/// 获取统计信息命令
#[tauri::command]
fn get_stats(state: State<'_, AppState>) -> Result<StatsInfo, String> {
    let app = state.app.lock().map_err(|e| e.to_string())?;
    let stats = app.stats();
    
    Ok(StatsInfo {
        data_count: stats.data_count,
        index_count: stats.index_count,
        message_count: stats.message_count,
    })
}

/// 发送消息命令
#[tauri::command]
fn send_message(
    state: State<'_, AppState>,
    token: String,
    recipient_id: String,
    title: String,
    content: String,
) -> Result<bool, String> {
    let mut app = state.app.lock().map_err(|e| e.to_string())?;
    
    app.send_message(&token, &recipient_id, &title, &content)
        .map_err(|e| e.to_string())?;
    
    Ok(true)
}

/// 获取用户消息命令
#[tauri::command]
fn get_messages(state: State<'_, AppState>, user_id: String, limit: usize) -> Result<Vec<MessageInfo>, String> {
    let app = state.app.lock().map_err(|e| e.to_string())?;
    
    let messages = app.get_messages(&user_id, limit);
    let message_infos: Vec<MessageInfo> = messages.into_iter().map(|m| {
        MessageInfo {
            id: m.id.to_string(),
            from: m.from.clone(),
            to: m.to.clone(),
            title: m.title.clone(),
            content: m.content.clone(),
            timestamp: m.timestamp.to_string(),
        }
    }).collect();
    
    Ok(message_infos)
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

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            app: Mutex::new(
                SynapseApp::new_local("/tmp/synapse-data").expect("Failed to create SynapseApp")
            ),
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
