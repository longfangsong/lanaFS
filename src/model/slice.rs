use crate::infrastructure::db::DB;
use crate::model::file::File;
use crate::model::server::{next_server_id, Server, SERVERS};
use bytes::Bytes;
use std::fs;
use std::fs::OpenOptions;
use std::io::{Seek, Write};

#[derive(Debug, Clone)]
pub struct Slice {
    pub id: u32,
}

impl Slice {
    pub async fn upload(belong_to: &File, into_server: &Server, content: Vec<u8>) -> Self {
        let db = DB.lock().unwrap();
        db.execute("INSERT INTO slice(from_file) VALUES (?);", &[belong_to.id])
            .unwrap();
        let id: u32 = db.last_insert_rowid() as _;
        db.execute(
            "INSERT INTO slice_server(slice_id, server_id)\
                    VALUES (?, ?);",
            &[id, into_server.id],
        )
        .unwrap();
        drop(db);
        into_server.upload(&format!("{}.slice", id), content).await;
        Slice { id }
    }
    pub async fn download(&self) -> Bytes {
        let server_id = DB
            .lock()
            .unwrap()
            .query_row(
                "SELECT server_id \
            FROM slice_server WHERE slice_id=?",
                &[self.id],
                |rows| rows.get(0),
            )
            .unwrap();
        SERVERS
            .get(&server_id)
            .unwrap()
            .download(&format!("{}.slice", self.id))
            .await
    }
}
