use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::Mutex;

use anyhow::Result;
use russh::*;
use serde_json::json;

use crate::session::{Client, Session};
use crate::utils::ApiResponse;

// 定义全局连接池
static CONNECTION_POOL: Lazy<Mutex<Vec<(usize, Session)>>> = Lazy::new(|| Mutex::new(Vec::new()));

#[derive(Debug, serde::Deserialize)]
pub enum SshCommand {
    OpenConnection { id: usize, url: String },
    CloseConnection(usize),
    ExecuteQuery { id: usize, query: String },
}

#[tauri::command]
pub async fn handle_ssh_command(command: SshCommand) -> Result<serde_json::Value, String> {
    println!("Received command: {:?}", command);
    match command {
        SshCommand::OpenConnection { id, url } => open_connection(id, url).await,
        SshCommand::CloseConnection(id) => close_connection(id).await,
        SshCommand::ExecuteQuery { id, query } => {
            if query == "baseinfo" {
                query_server_info(id).await
            } else {
                execute_query(id, query).await
            }
        }
    }
}

async fn open_connection(id: usize, url: String) -> Result<serde_json::Value, String> {
    // 解析 URL
    let parts: Vec<&str> = url.split('@').collect();
    if parts.len() != 2 {
        return Err("无效的 URL 格式".into());
    }

    let credentials: Vec<&str> = parts[0].split(':').collect();
    let address: Vec<&str> = parts[1].split(':').collect();

    if credentials.len() != 2 || address.len() != 2 {
        return Err("无效的 URL 格式".into());
    }

    let username = credentials[0];
    let password = credentials[1];
    let host = address[0];
    let port: u16 = address[1].parse().map_err(|_| "无效的端口号")?;

    // 创建 SSH 连接
    let config = Arc::new(client::Config::default());
    let sh = Client {};
    let mut session = client::connect(config, (host, port), sh)
        .await
        .map_err(|e| {
            format!(
                "连接失败, {}",
                e.to_string()
                    .split_whitespace()
                    .next()
                    .unwrap_or("未知错误")
            )
        })?;
    let auth_res = session
        .authenticate_password(username, password)
        .await
        .map_err(|e| {
            format!(
                "身份验证失败, {}",
                e.to_string()
                    .split_whitespace()
                    .next()
                    .unwrap_or("未知错误")
            )
        })?;

    if !auth_res {
        return Err("身份验证失败".into());
    }

    // 将连接添加到全局连接池
    let new_session = Session { session };
    CONNECTION_POOL.lock().await.push((id, new_session));

    // 这里可以添加更多的连接管理逻辑，比如限制连接池大小、处理连接超时等

    Ok(json!(ApiResponse::success(json!({
        "id": id,
        "message": format!("连接到: {}", parts[1])
    }))))
}

async fn close_connection(id: usize) -> Result<serde_json::Value, String> {
    let mut pool = CONNECTION_POOL.lock().await;
    if let Some(index) = pool.iter().position(|(conn_id, _)| *conn_id == id) {
        pool.remove(index);
        Ok(json!(ApiResponse::success(format!(
            "已关闭连接 ID: {}",
            id
        ))))
    } else {
        Err(format!("未找到 ID 为 {} 的连接", id))
    }
}

async fn execute_query(connection_id: usize, query: String) -> Result<serde_json::Value, String> {
    let pool = CONNECTION_POOL.lock().await;
    if let Some((_, session)) = pool.iter().find(|(id, _)| *id == connection_id) {
        let result = session.call(&query).await.map_err(|e| e.to_string())?;
        Ok(json!(ApiResponse::success(result)))
    } else {
        Err(format!("未找到 ID 为 {} 的连接", connection_id))
    }
}

async fn query_server_info(connection_id: usize) -> Result<serde_json::Value, String> {
    let pool = CONNECTION_POOL.lock().await;
    if let Some((_, session)) = pool.iter().find(|(id, _)| *id == connection_id) {
        let uptime = session.call("uptime").await.map_err(|e| e.to_string())?;
        let memory_usage = session.call("free -m").await.map_err(|e| e.to_string())?;
        let cpu_usage = session
            .call("top -bn1 | grep 'Cpu(s)'")
            .await
            .map_err(|e| e.to_string())?;

        Ok(json!(ApiResponse::success(json!({
            "uptime": uptime,
            "memory_usage": memory_usage,
            "cpu_usage": cpu_usage
        }))))
    } else {
        Err(format!("未找到 ID 为 {} 的连接", connection_id))
    }
}

async fn list_connections() -> Result<serde_json::Value, String> {
    let pool = CONNECTION_POOL.lock().await;
    let connections: Vec<usize> = pool.iter().map(|(id, _)| *id).collect();
    Ok(json!(ApiResponse::success(connections)))
}
