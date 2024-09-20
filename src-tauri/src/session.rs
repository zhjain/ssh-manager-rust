use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use russh::client::{self, Handle};
use russh::keys::*;
use tauri::Window;
use tokio::net::ToSocketAddrs;
use tokio::task::JoinHandle;
use tokio::time::interval;

pub struct Client {}

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

/// 这个结构体是russh客户端的便捷包装
pub struct Session {
    pub session: Handle<Client>,
    pub info_task: Option<JoinHandle<()>>,
}

impl Session {
    /// 使用私钥连接到服务器
    pub async fn connect<P: AsRef<Path>, A: ToSocketAddrs>(
        key_path: P,
        user: impl Into<String>,
        addrs: A,
    ) -> Result<Self> {
        let key_pair = load_secret_key(key_path, None)?;
        let config = client::Config {
            inactivity_timeout: Some(Duration::from_secs(5)),
            ..Default::default()
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

        Ok(Self {
            session,
            info_task: None,
        })
    }

    /// 使用用户名和密码连接到服务器
    pub async fn connect_with_password<A: ToSocketAddrs>(
        user: impl Into<String>,
        password: impl Into<String>,
        addrs: A,
    ) -> Result<Self> {
        let config = client::Config {
            inactivity_timeout: Some(Duration::from_secs(5)),
            ..Default::default()
        };

        let config = Arc::new(config);
        let sh = Client {};

        let mut session = client::connect(config, addrs, sh).await?;
        let auth_res = session.authenticate_password(user, password).await?;

        if !auth_res {
            anyhow::bail!("密码认证失败");
        }

        Ok(Self {
            session,
            info_task: None,
        })
    }

    /// 在服务器上执行命令
    pub async fn call(&self, command: &str) -> Result<String> {
        let mut channel = self.session.channel_open_session().await?;
        channel.exec(true, command).await?;

        let mut output = String::new();
        loop {
            let Some(msg) = channel.wait().await else {
                break;
            };
            match msg {
                russh::ChannelMsg::Data { ref data } => {
                    output.push_str(&String::from_utf8_lossy(data));
                }
                russh::ChannelMsg::ExitStatus { exit_status } => {
                    if exit_status != 0 {
                        anyhow::bail!("命令返回非零退出状态");
                    }
                }
                _ => {}
            }
        }

        Ok(output)
    }

    /// 关闭会话
    pub async fn close(&mut self) -> Result<()> {
        if let Some(task) = self.info_task.take() {
            task.abort();
        }
        self.session
            .disconnect(russh::Disconnect::ByApplication, "", "English")
            .await?;
        Ok(())
    }

    async fn exec_command(session: &Handle<Client>, command: &str) -> Result<String> {
        let mut channel = session.channel_open_session().await?;
        channel.exec(true, command).await?;

        let mut output = String::new();
        while let Some(msg) = channel.wait().await {
            if let russh::ChannelMsg::Data { ref data } = msg {
                output.push_str(&String::from_utf8_lossy(data));
            }
        }

        Ok(output)
    }
}

// fn load_secret_key<P: AsRef<Path>>(
//     _key_path: P,
//     _passphrase: Option<&str>,
// ) -> Result<keys::KeyPair> {
//     // 这里你需要实现从文件加载密钥的逻辑
//     // 这个函数的返回值是 russh_keys::key::KeyPair
//     todo!()
// }
