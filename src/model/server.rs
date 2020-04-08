use crate::infrastructure::db::DB;
use bytes::Bytes;
use reqwest::Client;
use rusqlite::NO_PARAMS;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use tokio::sync::Semaphore;

pub struct Server {
    pub id: u32,
    pub address: String,
    pub client: Client,
    free_connection: Semaphore,
}

lazy_static! {
    pub static ref SERVERS: HashMap<u32, Server> = {
        let mut result = HashMap::new();
        let db = DB.lock().unwrap();
        let mut stmt = db.prepare("SELECT * FROM server;").unwrap();
        let servers = stmt
            .query_map(NO_PARAMS, |row| {
                let id: u32 = row.get(0).unwrap();
                let address: String = row.get(1).unwrap();
                Ok(Server {
                    id,
                    address,
                    client: reqwest::Client::builder().referer(false).build().unwrap(),
                    free_connection: Semaphore::new(2),
                })
            })
            .unwrap();
        for server in servers {
            let server = server.unwrap();
            result.insert(server.id, server);
        }
        result
    };
    static ref next_id: AtomicU32 = AtomicU32::new(0);
}

impl Server {
    pub async fn download(&self, filename: &str) -> Bytes {
        let block = self.free_connection.acquire().await;
        let result = self
            .client
            .get(&self.address)
            .query(&[("name", filename)])
            .send()
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap();
        drop(block);
        result
    }
    pub async fn upload(&self, filename: &str, content: Vec<u8>) -> bool {
        let block = self.free_connection.acquire().await;
        let result = self
            .client
            .post(&self.address)
            .query(&[("name", filename)])
            .body(content)
            .send()
            .await
            .unwrap()
            .status()
            == 202;
        drop(block);
        result
    }
}

pub fn next_server_id() -> u32 {
    let result = next_id.load(Ordering::Relaxed);
    let next = (result + 1) % SERVERS.len() as u32;
    next_id.store(next, Ordering::Relaxed);
    result + 1
}

pub fn next_server() -> &'static Server {
    let server_id = next_server_id();
    SERVERS.get(&server_id).unwrap()
}
