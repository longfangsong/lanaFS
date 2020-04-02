use crate::infrastructure::db::DB;
use crate::model::slice::Slice;
use std::fs;

pub struct File {
    pub id: u32,
}

impl File {
    pub fn new(filename: &str) -> Self {
        let db = DB.lock().unwrap();
        db.execute(
            "INSERT INTO file(name)\
            VALUES (?);",
            &[filename],
        )
            .unwrap();

        let id: u32 = db.last_insert_rowid() as _;

        File { id }
    }
    pub fn find(filename: &str) -> Option<Self> {
        let db = DB.lock().unwrap();
        db.query_row("SELECT id FROM file WHERE name=?;", &[filename], |row| {
            row.get(0)
        })
            .map(|id| File { id })
            .ok()
    }
    pub fn filename(&self) -> String {
        let db = DB.lock().unwrap();
        db.query_row("SELECT name FROM file WHERE id=?;", &[self.id], |row| {
            row.get(0)
        })
            .unwrap()
    }
    pub fn slices(&self) -> Vec<Slice> {
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
