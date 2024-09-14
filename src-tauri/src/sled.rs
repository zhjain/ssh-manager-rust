use serde::{Deserialize, Serialize};
use serde_json::json;
use sled::{self, Db};

use crate::utils::ApiResponse;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RedisConnection {
    id: Option<u64>,
    name: String,
    host: String,
    port: u16,
    username: Option<String>,
    password: Option<String>,
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

#[derive(Serialize, Deserialize, Debug)]
pub enum DbOperation {
    Insert(RedisConnection),
    Update(RedisConnection),
    Delete(u64), // 传入要删除的记录的 ID
    SelectAll,   // 查询所有记录
}

#[tauri::command]
pub async fn handle_db_operation(operation: DbOperation) -> Result<serde_json::Value, String> {
    println!("Received operation: {:?}", operation); // 打印操作类型
    let db: Db = sled::open("sled_db/connections_db").map_err(|e| e.to_string())?;

    match operation {
        DbOperation::Insert(mut connection) => {
            save_connection(&db, &mut connection).map_err(|e| e.to_string())?;
            let connections = get_all_connections(&db).map_err(|e| e.to_string())?;
            Ok(json!(ApiResponse::success(connections)))
        }
        DbOperation::Update(mut connection) => {
            update_connection(&db, &mut connection).map_err(|e| e.to_string())?;
            let connections = get_all_connections(&db).map_err(|e| e.to_string())?;
            Ok(json!(ApiResponse::success(connections)))
        }
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

fn save_connection(
    db: &Db,
    connection: &mut RedisConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    let new_id = db.generate_id()?;
    connection.id = Some(new_id); // 将生成的 ID 分配给连接
    let key = connection.id.unwrap().to_string();
    let value = serde_json::to_string(&connection)?.into_bytes();
    db.insert(key, value)?;
    Ok(())
}

fn get_all_connections(db: &Db) -> Result<Vec<RedisConnection>, Box<dyn std::error::Error>> {
    let mut connections = Vec::new();
    for item in db.iter() {
        let (_, value) = item?;
        let connection: RedisConnection = serde_json::from_slice(&value)?;
        connections.push(connection);
    }
    Ok(connections)
}

fn update_connection(
    db: &Db,
    connection: &mut RedisConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    save_connection(db, connection)
}

fn delete_connection(db: &Db, id: u64) -> Result<(), Box<dyn std::error::Error>> {
    db.remove(id.to_string())?;
    Ok(())
}
