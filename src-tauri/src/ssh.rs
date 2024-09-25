use once_cell::sync::Lazy;
use std::sync::Arc;
use tauri::Window;
use tokio::sync::Mutex;

use anyhow::Result;
use serde_json::json;

use crate::session::Session;
use crate::utils::ApiResponse;

// 定义全局连接池
static CONNECTION_POOL: Lazy<Mutex<Vec<(usize, Session)>>> = Lazy::new(|| Mutex::new(Vec::new()));

#[derive(Debug, serde::Deserialize)]
pub enum SshCommand {
    OpenConnection { id: usize, url: String },
    CloseConnection(usize),
    ExecuteQuery { id: usize, query: String },
    RetryInfoQuery(usize),
    CloseAllConnections,
}

#[tauri::command]
pub async fn handle_ssh_command(
    command: SshCommand,
    window: Window,
) -> Result<serde_json::Value, String> {
    println!("收到SSH命令: {:?}", command);
    match command {
        SshCommand::OpenConnection { id, url } => open_connection(id, url, window).await,
        SshCommand::CloseConnection(id) => close_connection(id).await,
        SshCommand::ExecuteQuery { id, query } => {
            if query == "baseinfo" {
                query_server_info(id).await
            } else {
                execute_query(id, query).await
            }
        }
        SshCommand::RetryInfoQuery(id) => retry_info_query(id, window).await,
        SshCommand::CloseAllConnections => close_all_connections().await,
    }
}

async fn open_connection(
    id: usize,
    url: String,
    window: Window,
) -> Result<serde_json::Value, String> {
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

    // 将连接添加到全局连接池
    let new_session = Session::connect_with_password(username, password, (host, port))
        .await
        .map_err(|e| e.to_string())?;
    let mut pool = CONNECTION_POOL.lock().await;
    pool.push((id, new_session));
    if let Some((_, session)) = pool.iter_mut().find(|(conn_id, _)| *conn_id == id) {
        let info_task = tokio::spawn(start_info_query(id, window.clone()));
        session.info_task = Some(info_task);
    }

    // 这里可以添加更多的连接管理逻辑，比如限制连接池大小、处理连接超时等

    Ok(json!(ApiResponse::success(json!({
        "id": id,
        "message": format!("连接到: {}", parts[1])
    }))))
}

async fn close_connection(id: usize) -> Result<serde_json::Value, String> {
    let mut pool = CONNECTION_POOL.lock().await;
    if let Some(index) = pool.iter().position(|(conn_id, _)| *conn_id == id) {
        let (_, mut session) = pool.remove(index);
        if let Some(info_task) = session.info_task.take() {
            info_task.abort();
        }
        session.close().await.map_err(|e| e.to_string())?;
        Ok(json!(ApiResponse::success(format!(
            "已关闭连接 ID: {}",
            id
        ))))
    } else {
        Err(format!("未找到 ID 为 {} 的连接", id))
    }
}

async fn close_all_connections() -> Result<serde_json::Value, String> {
    let mut pool = CONNECTION_POOL.lock().await;
    let mut count = 0;
    for (_, session) in pool.iter_mut() {
        session.close().await.map_err(|e| e.to_string())?;
        count += 1;
    }
    pool.clear();
    println!("已关闭 {} 个连接", count);
    Ok(json!(ApiResponse::success("已关闭所有连接")))
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
    println!("查询服务器信息, id: {}", connection_id);
    let pool = CONNECTION_POOL.lock().await;
    if let Some((_, session)) = pool.iter().find(|(id, _)| *id == connection_id) {
        let uptime = session.call("uptime").await.map_err(|e| e.to_string())?;
        let uptime_parts: Vec<&str> = uptime.split(',').collect();
        let uptime_value = uptime_parts.get(0).map(|s| s.trim()).unwrap_or("");

        let memory_info = session.call("free -m").await.map_err(|e| e.to_string())?;
        let memory_lines: Vec<&str> = memory_info.lines().collect();
        let memory_parts: Vec<&str> = memory_lines
            .get(1)
            .unwrap_or(&"")
            .split_whitespace()
            .collect();
        let memory_usage = json!({
            "total": memory_parts.get(1).unwrap_or(&"0").parse::<u64>().unwrap_or(0),
            "used": memory_parts.get(2).unwrap_or(&"0").parse::<u64>().unwrap_or(0),
            "free": memory_parts.get(3).unwrap_or(&"0").parse::<u64>().unwrap_or(0)
        });

        let cpu_info = session
            .call("top -bn1 | grep 'Cpu(s)'")
            .await
            .map_err(|e| e.to_string())?;
        let cpu_parts: Vec<&str> = cpu_info.split(':').collect();
        let cpu_percentages: Vec<&str> = cpu_parts.get(1).unwrap_or(&"").split(',').collect();
        let user_cpu = cpu_percentages
            .get(0)
            .unwrap_or(&"0%")
            .trim()
            .trim_end_matches('%')
            .parse::<f32>()
            .unwrap_or(0.0);
        let system_cpu = cpu_percentages
            .get(1)
            .unwrap_or(&"0%")
            .trim()
            .trim_end_matches('%')
            .parse::<f32>()
            .unwrap_or(0.0);
        let cpu_usage = json!({
            "user": user_cpu,
            "system": system_cpu,
            "total": user_cpu + system_cpu
        });

        // 查询网络速率
        let net_info_before = session
            .call("cat /proc/net/dev")
            .await
            .map_err(|e| e.to_string())?;
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        let net_info_after = session
            .call("cat /proc/net/dev")
            .await
            .map_err(|e| e.to_string())?;

        let (rx_before, tx_before) = parse_net_info(&net_info_before);
        let (rx_after, tx_after) = parse_net_info(&net_info_after);

        let rx_speed = (rx_after.saturating_sub(rx_before) as f64) / 1024.0; // 转换为kb/s
        let tx_speed = (tx_after.saturating_sub(tx_before) as f64) / 1024.0; // 转换为kb/s

        // 查询磁盘空间使用情况
        let disk_info = session
            .call("df -h --total")
            .await
            .map_err(|e| e.to_string())?;
        let disk_lines: Vec<&str> = disk_info.lines().collect();
        let total_parts: Vec<&str> = disk_lines
            .last()
            .unwrap_or(&"")
            .split_whitespace()
            .collect();
        let total_used = total_parts
            .get(2)
            .unwrap_or(&"0")
            .parse::<u64>()
            .unwrap_or(0);
        let total_available = total_parts
            .get(3)
            .unwrap_or(&"0")
            .parse::<u64>()
            .unwrap_or(0);

        let network_usage = json!({
            "rx_speed": rx_speed,
            "tx_speed": tx_speed
        });

        Ok(json!(ApiResponse::success(json!({
            "uptime": uptime_value,
            "memory_usage": memory_usage,
            "cpu_usage": cpu_usage,
            "network_usage": network_usage,
            "disk_usage": {
                "used": total_used,
                "available": total_available
            },
            "upload_speed": tx_speed,
            "download_speed": rx_speed
        }))))
    } else {
        Err(format!("未找到 ID 为 {} 的连接", connection_id))
    }
}

fn parse_net_info(info: &str) -> (u64, u64) {
    let mut rx_bytes = 0;
    let mut tx_bytes = 0;
    for line in info.lines() {
        if line.contains("eth0") || line.contains("ens") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() > 9 {
                rx_bytes += parts[1].parse::<u64>().unwrap_or(0);
                tx_bytes += parts[9].parse::<u64>().unwrap_or(0);
            }
        }
    }
    (rx_bytes, tx_bytes)
}
async fn start_info_query(id: usize, window: Window) {
    let mut uptime_interval = tokio::time::interval(std::time::Duration::from_secs(60));
    let mut memory_interval = tokio::time::interval(std::time::Duration::from_secs(5));
    let mut cpu_interval = tokio::time::interval(std::time::Duration::from_secs(2));
    let mut network_interval = tokio::time::interval(std::time::Duration::from_secs(1));
    let mut disk_interval = tokio::time::interval(std::time::Duration::from_secs(30)); // Increased interval for disk usage

    loop {
        tokio::select! {
            _ = uptime_interval.tick() => {
                if let Err(e) = query_and_emit_uptime(id, &window).await {
                    eprintln!("查询运行时间失败: {}", e);
                }
            }
            _ = memory_interval.tick() => {
                if let Err(e) = query_and_emit_memory(id, &window).await {
                    eprintln!("查询内存使用失败: {}", e);
                }
            }
            _ = cpu_interval.tick() => {
                if let Err(e) = query_and_emit_cpu(id, &window).await {
                    eprintln!("查询CPU使用失败: {}", e);
                }
            }
            _ = network_interval.tick() => {
                if let Err(e) = query_and_emit_network(id, &window).await {
                    eprintln!("查询网络使用失败: {}", e);
                }
            }
            _ = disk_interval.tick() => {
                if let Err(e) = query_and_emit_disk(id, &window).await {
                    eprintln!("查询磁盘使用失败: {}", e);
                }
            }
        }
    }
}

async fn query_and_emit_uptime(id: usize, window: &Window) -> Result<(), String> {
    let pool = CONNECTION_POOL.lock().await;
    if let Some((_, session)) = pool.iter().find(|(conn_id, _)| *conn_id == id) {
        let uptime = session.call("uptime").await.map_err(|e| e.to_string())?;
        let uptime_parts: Vec<&str> = uptime.split(',').collect();
        let uptime_value = uptime_parts.get(0).map(|s| s.trim()).unwrap_or("");
        let _ = window.emit(
            &format!("server-uptime-update-{id}"),
            json!({ "uptime": uptime_value }),
        );
    }
    Ok(())
}

async fn query_and_emit_memory(id: usize, window: &Window) -> Result<(), String> {
    let pool = CONNECTION_POOL.lock().await;
    if let Some((_, session)) = pool.iter().find(|(conn_id, _)| *conn_id == id) {
        let memory_info = session.call("free -m").await.map_err(|e| e.to_string())?;
        let memory_lines: Vec<&str> = memory_info.lines().collect();
        let memory_parts: Vec<&str> = memory_lines
            .get(1)
            .unwrap_or(&"")
            .split_whitespace()
            .collect();
        let memory_usage = json!({
            "total": memory_parts.get(1).unwrap_or(&"0").parse::<u64>().unwrap_or(0),
            "used": memory_parts.get(2).unwrap_or(&"0").parse::<u64>().unwrap_or(0),
            "free": memory_parts.get(3).unwrap_or(&"0").parse::<u64>().unwrap_or(0)
        });
        let _ = window.emit(
            &format!("server-memory-update-{id}"),
            json!({ "memory_usage": memory_usage }),
        );
    }
    Ok(())
}

async fn query_and_emit_cpu(id: usize, window: &Window) -> Result<(), String> {
    let pool = CONNECTION_POOL.lock().await;
    if let Some((_, session)) = pool.iter().find(|(conn_id, _)| *conn_id == id) {
        let cpu_info = session
            .call("top -bn1 | grep 'Cpu(s)'")
            .await
            .map_err(|e| e.to_string())?;
        let cpu_parts: Vec<&str> = cpu_info.split(':').collect();
        let cpu_percentages: Vec<&str> = cpu_parts.get(1).unwrap_or(&"").split(',').collect();
        let user_cpu = cpu_percentages
            .get(0)
            .unwrap_or(&"0%")
            .trim()
            .trim_end_matches('%')
            .parse::<f32>()
            .unwrap_or(0.0);
        let system_cpu = cpu_percentages
            .get(1)
            .unwrap_or(&"0%")
            .trim()
            .trim_end_matches('%')
            .parse::<f32>()
            .unwrap_or(0.0);
        let cpu_usage = json!({
            "user": user_cpu,
            "system": system_cpu,
            "total": user_cpu + system_cpu
        });
        let _ = window.emit(
            &format!("server-cpu-update-{id}"),
            json!({ "cpu_usage": cpu_usage }),
        );
    }
    Ok(())
}

async fn query_and_emit_network(id: usize, window: &Window) -> Result<(), String> {
    let pool = CONNECTION_POOL.lock().await;
    if let Some((_, session)) = pool.iter().find(|(conn_id, _)| *conn_id == id) {
        let net_info_before = session
            .call("cat /proc/net/dev")
            .await
            .map_err(|e| e.to_string())?;
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        let net_info_after = session
            .call("cat /proc/net/dev")
            .await
            .map_err(|e| e.to_string())?;

        let (rx_before, tx_before) = parse_net_info(&net_info_before);
        let (rx_after, tx_after) = parse_net_info(&net_info_after);

        let rx_speed = (rx_after.saturating_sub(rx_before) as f64) / 1024.0; // 转换为kb/s
        let tx_speed = (tx_after.saturating_sub(tx_before) as f64) / 1024.0; // 转换为kb/s

        let network_usage = json!({
            "rx_speed": rx_speed,
            "tx_speed": tx_speed
        });
        let _ = window.emit(
            &format!("server-network-update-{id}"),
            json!({ "network_usage": network_usage }),
        );
    }
    Ok(())
}

async fn query_and_emit_disk(id: usize, window: &Window) -> Result<(), String> {
    let pool = CONNECTION_POOL.lock().await;
    if let Some((_, session)) = pool.iter().find(|(conn_id, _)| *conn_id == id) {
        let disk_info = session
            .call("df -h --total")
            .await
            .map_err(|e| e.to_string())?;
        let disk_lines: Vec<&str> = disk_info.lines().collect();
        let total_parts: Vec<&str> = disk_lines
            .last()
            .unwrap_or(&"")
            .split_whitespace()
            .collect();

        let total_size = total_parts.get(1).unwrap_or(&"0").to_string();
        let total_used = total_parts.get(2).unwrap_or(&"0").to_string();
        let total_available = total_parts.get(3).unwrap_or(&"0").to_string();
        let use_percentage = total_parts.get(4).unwrap_or(&"0%").to_string();

        let _ = window.emit(
            &format!("server-disk-update-{id}"),
            json!({
                "disk_usage": {
                    "total": total_size,
                    "used": total_used,
                    "available": total_available,
                    "use_percentage": use_percentage
                }
            }),
        );
    }
    Ok(())
}

async fn retry_info_query(id: usize, window: Window) -> Result<serde_json::Value, String> {
    let mut pool = CONNECTION_POOL.lock().await;
    if let Some((_, session)) = pool.iter_mut().find(|(conn_id, _)| *conn_id == id) {
        if let Some(old_task) = session.info_task.take() {
            old_task.abort();
        }
        let new_task = tokio::spawn(start_info_query(id, window));
        session.info_task = Some(new_task);
        Ok(json!(ApiResponse::success("重新开始查询服务器信息")))
    } else {
        Err(format!("未找到 ID 为 {} 的连接", id))
    }
}
