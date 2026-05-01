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
fn login(state: State<'_, AppState>, username: String, password: String) -> Result<String, String> {
    let app = state.app.lock().map_err(|e| e.to_string())?;
    
    // 简化版本：直接返回 token
    // 生产环境应使用异步运行时
    Ok(format!("token-{}-{}", username, password.len()))
}

/// 存储数据命令
#[tauri::command]
fn store_data(
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
    
    // 简化版本：同步执行
    // 生产环境应使用 tokio::spawn
    let entity = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(app.secure_store(&token, dt, content.into_bytes(), tags))
        .map_err(|e| e.to_string())?;
    
    Ok(entity.id.to_string())
}

/// 获取数据命令
#[tauri::command]
fn get_data(state: State<'_, AppState>, token: String, id: String) -> Result<String, String> {
    let app = state.app.lock().map_err(|e| e.to_string())?;
    
    let entity = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(app.secure_get(&token, &id))
        .map_err(|e| e.to_string())?;
    
    Ok(String::from_utf8_lossy(&entity.encrypted_content).to_string())
}

/// 搜索数据命令
#[tauri::command]
fn search_data(state: State<'_, AppState>, query: String, limit: usize) -> Result<Vec<String>, String> {
    let app = state.app.lock().map_err(|e| e.to_string())?;
    
    let results = app.search(&query, limit);
    let ids: Vec<String> = results.into_iter().map(|r| r.id).collect();
    
    Ok(ids)
}

/// 删除数据命令
#[tauri::command]
fn delete_data(state: State<'_, AppState>, token: String, id: String) -> Result<bool, String> {
    let mut app = state.app.lock().map_err(|e| e.to_string())?;
    
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(app.secure_delete(&token, &id))
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

/// 统计信息
#[derive(serde::Serialize)]
struct StatsInfo {
    data_count: usize,
    index_count: usize,
    message_count: usize,
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            app: Mutex::new(
                SynapseApp::new("/tmp/synapse-data").expect("Failed to create SynapseApp")
            ),
        })
        .invoke_handler(tauri::generate_handler![
            login,
            store_data,
            get_data,
            search_data,
            delete_data,
            get_stats,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
