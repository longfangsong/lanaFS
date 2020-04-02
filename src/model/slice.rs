use crate::infrastructure::db::DB;
use crate::model::file::File;
use crate::model::server::{Server, SERVERS};
use crate::rpc::rpc::DownloadRequest;
use futures::compat::Future01CompatExt;
use futures::io::SeekFrom;
use std::fs;
use std::fs::OpenOptions;
use std::io::{Seek, Write};

#[derive(Debug, Clone)]
pub struct Slice {
    pub id: u32,
}

impl Slice {
    pub fn new(from: &File) -> Self {
        let db = DB.lock().unwrap();
        db.execute(
            "INSERT INTO slice(from_file) VALUES (?);
        ",
            &[from.id],
        )
        .unwrap();
        let id: u32 = db.last_insert_rowid() as _;
        Slice { id }
    }
    pub async fn download(&self) -> fs::File {
        let in_server = DB
            .lock()
            .unwrap()
            .query_row(
                "SELECT server_id FROM slice_server WHERE slice_id=?;",
                &[self.id],
                |row| {
                    let result: u32 = row.get(0).unwrap();
                    Ok(result)
                },
            )
            .unwrap();
        let server: Server = SERVERS.get(&in_server).unwrap().clone();
        let mut request = DownloadRequest::new();
        request.set_Filename(format!("{}.slice", self.id));
        let data: Vec<u8> = server
            .client
            .download_async(&request)
            .unwrap()
            .compat()
            .await
            .unwrap()
            .take_data();
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(format!("/download/{}.slice", self.id))
            .unwrap();
        file.write_all(&data).unwrap();
        file.seek(SeekFrom::Start(0)).unwrap();
        file
    }
}
