use once_cell::sync::Lazy;
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use russh::keys::*;
use russh::*;
use tokio::io::AsyncWriteExt;
use tokio::net::ToSocketAddrs;

// 定义全局连接池
static CONNECTION_POOL: Lazy<Mutex<Vec<Session>>> = Lazy::new(|| Mutex::new(Vec::new()));
#[derive(Debug, serde::Deserialize)]
pub enum SshCommand {
    OpenConnection(String),
    CloseConnection(usize),
    ExecuteQuery { connection_id: usize, query: String },
}

#[tauri::command]
pub async fn handle_ssh_command(command: SshCommand) -> Result<String, String> {
    match command {
        SshCommand::OpenConnection(url) => open_connection(url).await,
        SshCommand::CloseConnection(id) => close_connection(id).await,
        SshCommand::ExecuteQuery {
            connection_id,
            query,
        } => execute_query(connection_id, query).await,
    }
}

async fn open_connection(url: String) -> Result<String, String> {
    // 解析 URL
    let parts: Vec<&str> = url.split('@').collect();
    if parts.len() != 2 {
        return Err("无效的 URL 格式".to_string());
    }

    let credentials: Vec<&str> = parts[0].split(':').collect();
    let address: Vec<&str> = parts[1].split(':').collect();

    if credentials.len() != 2 || address.len() != 2 {
        return Err("无效的 URL 格式".to_string());
    }

    let username = credentials[0];
    let password = credentials[1];
    let host = address[0];
    let port: u16 = address[1].parse().map_err(|_| "无效的端口号".to_string())?;

    // 创建 SSH 连接
    let config = Arc::new(client::Config::default());
    let sh = Client {};
    let mut session = client::connect(config, (host, port), sh)
        .await
        .map_err(|e| e.to_string())?;

    let auth_res = session
        .authenticate_password(username, password)
        .await
        .map_err(|e| e.to_string())?;

    if !auth_res {
        return Err("身份验证失败".to_string());
    }

    // 将连接添加到全局连接池
    let new_session = Session { session };
    CONNECTION_POOL.lock().unwrap().push(new_session);

    // 这里可以添加更多的连接管理逻辑，比如限制连接池大小、处理连接超时等

    Ok(format!("连接到: {}", url))
}

async fn close_connection(id: usize) -> Result<String, String> {
    // 实现关闭连接的逻辑
    Err("尚未实现".to_string())
}

async fn execute_query(connection_id: usize, query: String) -> Result<String, String> {
    // 实现执行查询的逻辑
    Err("尚未实现".to_string())
}

async fn list_connections() -> Result<String, String> {
    // 实现列出所有连接的逻辑
    Err("尚未实现".to_string())
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
            anyhow::bail!("Authentication failed");
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
        Ok(code.expect("program did not exit cleanly"))
    }

    async fn close(&mut self) -> Result<()> {
        self.session
            .disconnect(Disconnect::ByApplication, "", "English")
            .await?;
        Ok(())
    }
}
