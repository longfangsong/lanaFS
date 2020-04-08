use crate::infrastructure::db::DB;
use crate::model::server::next_server;
use crate::model::slice::Slice;
use futures::future::join_all;
use std::fs;
use std::io::{Read, Write};

#[derive(Clone)]
pub struct File {
    pub id: u32,
}

impl File {
    pub fn find(filename: &str) -> Option<Self> {
        DB.lock()
            .unwrap()
            .query_row("SELECT id FROM file WHERE name=?;", &[filename], |row| {
                row.get(0)
            })
            .map(|id| File { id })
            .ok()
    }

    pub async fn upload(filename: &str, content: &mut fs::File) -> Self {
        let db = DB.lock().unwrap();
        db.execute("INSERT INTO file(name) VALUES (?);", &[filename])
            .unwrap();
        let id: u32 = db.last_insert_rowid() as _;
        drop(db);
        let result = File { id };
        let mut upload_tasks = Vec::new();
        loop {
            let mut buffer = vec![0u8; 1024 * 1024 * 4];
            let amount = content.read(&mut buffer);
            if let Ok(read_amount) = amount {
                if read_amount == 0 {
                    break;
                } else {
                    upload_tasks
                        .push(async { Slice::upload(&result, next_server(), buffer).await });
                }
            } else {
                break;
            }
        }
        println!("{} tasks", upload_tasks.len());
        join_all(upload_tasks).await;
        result.clone()
    }

    pub async fn download(&self, to: &mut fs::File) {
        let slices = self.slices();
        let download_tasks = slices.iter().map(|it| it.download());
        let results = join_all(download_tasks).await;
        results.iter().for_each(|it| {
            to.write_all(it).unwrap();
        })
    }

    fn filename(&self) -> String {
        let db = DB.lock().unwrap();
        db.query_row("SELECT name FROM file WHERE id=?;", &[self.id], |row| {
            row.get(0)
        })
        .unwrap()
    }

    fn slices(&self) -> Vec<Slice> {
        let db = DB.lock().unwrap();
        let mut stmt = db
            .prepare("SELECT id FROM slice WHERE from_file=? ORDER BY id;")
            .unwrap();
        let result_ids = stmt
            .query_map(&[self.id], |row| row.get(0))
            .unwrap()
            .map(|it| it.unwrap());
        result_ids.map(|it: u32| Slice { id: it }).collect()
    }
}
