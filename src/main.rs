#[macro_use]
extern crate lazy_static;

use crate::model::file::File;
use std::path::Path;
use std::{env, fs};

mod infrastructure;
mod model;

#[tokio::main]
async fn main() {
    let mut args = env::args().skip(1);
    let function = args.next().unwrap();
    let param = args.next().unwrap();
    let filename = Path::new(&param).file_name().unwrap();
    if &function == "upload" {
        let mut raw_file = fs::File::open(&param).unwrap();
        File::upload(filename.to_str().unwrap(), &mut raw_file).await;
    } else if &function == "download" {
        let mut raw_file = fs::File::create(&param).unwrap();
        let file = File::find(&filename.to_str().unwrap()).unwrap();
        file.download(&mut raw_file).await;
    }
}
