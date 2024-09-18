use once_cell::sync::Lazy;
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use russh::keys::*;
use russh::*;
use serde_json::json;
use tokio::io::AsyncWriteExt;
use tokio::net::ToSocketAddrs;

use crate::utils::ApiResponse;

// 定义全局连接池
static CONNECTION_POOL: Lazy<Mutex<Vec<(usize, Session)>>> = Lazy::new(|| Mutex::new(Vec::new()));
static NEXT_ID: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));

#[derive(Debug, serde::Deserialize)]
pub enum SshCommand {
    OpenConnection(String),
    CloseConnection(usize),
    ExecuteQuery { connection_id: usize, query: String },
}

#[tauri::command]
pub async fn handle_ssh_command(command: SshCommand) -> Result<serde_json::Value, String> {
    println!("Received command: {:?}", command);
    match command {
        SshCommand::OpenConnection(url) => open_connection(url).await,
        SshCommand::CloseConnection(id) => close_connection(id).await,
        SshCommand::ExecuteQuery {
            connection_id,
            query,
        } => execute_query(connection_id, query).await,
    }
}

async fn open_connection(url: String) -> Result<serde_json::Value, String> {
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

    // 生成新的连接ID
    let id = {
        let mut next_id = NEXT_ID.lock().unwrap();
        *next_id += 1;
        *next_id
    };

    // 将连接添加到全局连接池
    let new_session = Session { session };
    CONNECTION_POOL.lock().unwrap().push((id, new_session));

    // 这里可以添加更多的连接管理逻辑，比如限制连接池大小、处理连接超时等

    Ok(json!(ApiResponse::success(json!({
        "id": id,
        "message": format!("连接到: {}", parts[1])
    }))))
}

async fn close_connection(id: usize) -> Result<serde_json::Value, String> {
    let mut pool = CONNECTION_POOL.lock().unwrap();
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
    let pool = CONNECTION_POOL.lock().unwrap();
    if let Some((_, session)) = pool.iter().find(|(id, _)| *id == connection_id) {
        // 这里实现执行查询的逻辑
        // 注意：这里需要修改 Session 结构体，添加执行查询的方法
        Err("执行查询功能尚未实现".into())
    } else {
        Err(format!("未找到 ID 为 {} 的连接", connection_id))
    }
}

async fn list_connections() -> Result<serde_json::Value, String> {
    let pool = CONNECTION_POOL.lock().unwrap();
    let connections: Vec<usize> = pool.iter().map(|(id, _)| *id).collect();
    Ok(json!(ApiResponse::success(connections)))
}

struct Client {}

// More SSH event handlers
// can be defined in this trait
// In this example, we're only using Channel, so these aren't needed.
#[async_trait]
impl client::Handler for Client {
    type Error = russh::Error;

    async fn check_server_key(
        &mut self,
        _server_public_key: &key::PublicKey,
    ) -> Result<bool, Self::Error> {
        Ok(true)
    }
}

/// This struct is a convenience wrapper
/// around a russh client
pub struct Session {
    session: client::Handle<Client>,
}

impl Session {
    async fn connect<P: AsRef<Path>, A: ToSocketAddrs>(
        key_path: P,
        user: impl Into<String>,
        addrs: A,
    ) -> Result<Self> {
        let key_pair = load_secret_key(key_path, None)?;
        let config = client::Config {
            inactivity_timeout: Some(Duration::from_secs(5)),
            ..<_>::default()
        };

        let config = Arc::new(config);
        let sh = Client {};

        let mut session = client::connect(config, addrs, sh).await?;
        let auth_res = session
            .authenticate_publickey(user, Arc::new(key_pair))
            .await?;

        if !auth_res {
            anyhow::bail!("身份验证失败");
        }

        Ok(Self { session })
    }

    async fn connect_with_password<A: ToSocketAddrs>(
        user: impl Into<String>,
        password: impl Into<String>,
        addrs: A,
    ) -> Result<Self> {
        let config = client::Config {
            inactivity_timeout: Some(Duration::from_secs(5)),
            ..<_>::default()
        };

        let config = Arc::new(config);
        let sh = Client {};

        let mut session = client::connect(config, addrs, sh).await?;
        let auth_res = session.authenticate_password(user, password).await?;

        if !auth_res {
            anyhow::bail!("密码认证失败");
        }

        Ok(Self { session })
    }

    async fn call(&mut self, command: &str) -> Result<u32> {
        let mut channel = self.session.channel_open_session().await?;
        channel.exec(true, command).await?;

        let mut code = None;
        let mut stdout = tokio::io::stdout();

        loop {
            // There's an event available on the session channel
            let Some(msg) = channel.wait().await else {
                break;
            };
            match msg {
                // Write data to the terminal
                ChannelMsg::Data { ref data } => {
                    stdout.write_all(data).await?;
                    stdout.flush().await?;
                }
                // The command has returned an exit code
                ChannelMsg::ExitStatus { exit_status } => {
                    code = Some(exit_status);
                    // cannot leave the loop immediately, there might still be more data to receive
                }
                _ => {}
            }
        }
        Ok(code.expect("程序未正常退出"))
    }

    async fn close(&mut self) -> Result<()> {
        self.session
            .disconnect(Disconnect::ByApplication, "", "English")
            .await?;
        Ok(())
    }
}
