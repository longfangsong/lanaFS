use crate::infrastructure::db::DB;
use crate::rpc::rpc_grpc::FileServiceClient;
use grpcio::{ChannelBuilder, EnvBuilder};
use rusqlite::NO_PARAMS;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

#[derive(Clone)]
pub struct Server {
    pub id: u32,
    pub client: FileServiceClient,
}

lazy_static! {
    pub static ref SERVERS: HashMap<u32, Server> = {
        let mut result = HashMap::new();
        let db = DB.lock().unwrap();
        let mut stmt = db.prepare("SELECT * FROM server;").unwrap();
        let kv = stmt
            .query_map(NO_PARAMS, |row| {
                let id: u32 = row.get(0).unwrap();
                let name: String = row.get(1).unwrap();
                Ok((id, name))
            })
            .unwrap();
        for item in kv {
            let (id, address) = item.unwrap();
            let env = Arc::new(EnvBuilder::new().build());
            let ch = ChannelBuilder::new(env).connect(&address);
            let client = FileServiceClient::new(ch);
            result.insert(id, Server { id, client });
        }
        result
    };
    static ref next_id: AtomicU32 = AtomicU32::new(0);
}

pub fn next_server_id() -> u32 {
    let result = next_id.load(Ordering::Relaxed);
    let next = (result + 1) % SERVERS.len() as u32;
    next_id.store(next, Ordering::Relaxed);
    result + 1
}
