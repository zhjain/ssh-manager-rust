use std::time::{SystemTime, UNIX_EPOCH};

use dirs;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sled::{self, Db};

use crate::utils::ApiResponse;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SshConnection {
    id: Option<u64>,
    name: String,
    host: String,
    port: u16,
    username: Option<String>,
    password: Option<String>,
    created_at: Option<u64>,
    updated_at: Option<u64>,
}

// impl RedisConnection {
//     fn new(
//         name: &str,
//         host: &str,
//         port: u16,
//         username: Option<&str>,
//         password: Option<&str>,
//     ) -> Self {
//         RedisConnection {
//             id: Some(0),
//             name: name.to_string(),
//             host: host.to_string(),
//             port,
//             username: username.map(|u| u.to_string()),
//             password: password.map(|p| p.to_string()),
//         }
//     }
// }

// 获取当前时间戳（秒）
fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DbOperation {
    Insert(SshConnection),
    Update(SshConnection),
    Delete(u64), // 传入要删除的记录的 ID
    SelectAll,   // 查询所有记录
}

#[tauri::command]
pub async fn handle_db_operation(operation: DbOperation) -> Result<serde_json::Value, String> {
    println!("Received db operation: {:?}", operation); // 打印操作类型
    let home_dir = dirs::home_dir().ok_or("无法获取用户主目录")?;
    let db_path = home_dir.join(".ssh-rust").join("connections_db");
    let db: Db = sled::open(db_path).map_err(|e| e.to_string())?;

    match operation {
        DbOperation::Insert(mut connection) => save_connection(&db, &mut connection),
        DbOperation::Update(mut connection) => update_connection(&db, &mut connection),
        DbOperation::Delete(id) => {
            delete_connection(&db, id).map_err(|e| e.to_string())?;
            let connections = get_all_connections(&db).map_err(|e| e.to_string())?;
            Ok(json!(ApiResponse::success(connections)))
        }
        DbOperation::SelectAll => {
            let connections = get_all_connections(&db).map_err(|e| e.to_string())?;
            Ok(json!(ApiResponse::success(connections)))
        }
    }
}

fn save_connection(db: &Db, connection: &mut SshConnection) -> Result<serde_json::Value, String> {
    let new_id = db.generate_id().map_err(|e| e.to_string())?;
    let create_at = current_timestamp();
    connection.id = Some(new_id); // 将生成的 ID 分配给连接
    connection.created_at = Some(create_at);
    connection.updated_at = Some(create_at);
    let key = connection.id.unwrap().to_string();
    let value = serde_json::to_string(&connection)
        .map_err(|e| e.to_string())?
        .into_bytes();
    db.insert(key, value).map_err(|e| e.to_string())?;
    println!("保存连接: {:?}", connection);
    Ok(json!(ApiResponse::success(connection)))
}

fn get_all_connections(db: &Db) -> Result<Vec<SshConnection>, Box<dyn std::error::Error>> {
    let mut connections = Vec::new();
    for item in db.iter() {
        let (_, value) = item?;
        let connection: SshConnection = serde_json::from_slice(&value)?;
        connections.push(connection);
    }
    connections.sort_by_key(|r| std::cmp::Reverse(r.created_at));
    Ok(connections)
}

fn update_connection(db: &Db, connection: &mut SshConnection) -> Result<serde_json::Value, String> {
    if let Some(id) = connection.id {
        let key = id.to_string();
        let update_at = current_timestamp();
        connection.updated_at = Some(update_at);
        let value = serde_json::to_string(&connection)
            .map_err(|e| e.to_string())?
            .into_bytes();
        db.insert(key, value).map_err(|e| e.to_string())?;
    } else {
        return Err("连接ID不存在".into());
    }
    Ok(json!(ApiResponse::success(connection)))
}

fn delete_connection(db: &Db, id: u64) -> Result<serde_json::Value, String> {
    db.remove(id.to_string()).map_err(|e| e.to_string())?;
    Ok(json!(ApiResponse::success(format!("成功删除ID为{}的连接", id))))
}
