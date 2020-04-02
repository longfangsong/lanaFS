#[macro_use]
extern crate lazy_static;

use crate::model::file::File;
use crate::service::{download, upload};
use futures::executor::block_on;
use std::fs;

mod infrastructure;
mod model;
mod rpc;
mod service;

fn main() {
    // let file = File::new("test.img");
    // let mut raw_file = fs::File::open("test.img").unwrap();
    // block_on(upload(&file, &mut raw_file));
    let file = File::find("test.img").unwrap();
    block_on(download(&file));
}
