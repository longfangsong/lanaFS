use crate::infrastructure::db::DB;
use crate::model::file::File;
use crate::model::server::next_server_id;
use crate::model::server::{Server, SERVERS};
use crate::model::slice::Slice;
use crate::rpc::rpc::UploadRequest;
use futures::compat::Future01CompatExt;
use futures::future::join_all;
use std::fs;
use std::io::{Read, Write};

async fn upload_slice(slice: Slice, data: Vec<u8>) {
    let upload_to_sever_id = next_server_id();
    DB.lock()
        .unwrap()
        .execute(
            "INSERT INTO slice_server(slice_id, server_id) VALUES (?, ?);",
            &[slice.id, upload_to_sever_id],
        )
        .unwrap();
    let upload_to_server: Server = SERVERS.get(&upload_to_sever_id).unwrap().clone();
    let mut request = UploadRequest::new();
    request.set_data(data);
    request.set_Filename(format!("{}.slice", slice.id));
    upload_to_server
        .client
        .upload_async(&request)
        .unwrap()
        .compat()
        .await
        .unwrap();
}

pub async fn upload(f: &File, raw_file: &mut fs::File) {
    let mut uploading = Vec::new();
    loop {
        let mut buffer = vec![0u8; 1024 * 1024];
        let read_length = raw_file.read(&mut buffer).unwrap();
        if read_length == 0 {
            break;
        }
        let slice = Slice::new(&f);
        uploading.push(upload_slice(slice, buffer));
    }
    join_all(uploading).await;
}

pub async fn download(f: &File) {
    let slices = f.slices();
    let tasks: Vec<_> = slices.iter().map(|slice| slice.download()).collect();
    let files: Vec<fs::File> = join_all(tasks).await;
    let mut result_file = fs::File::create(f.filename()).unwrap();
    let mut buffer = vec![0u8; 1024 * 1024];
    for mut file in files {
        let count = file.read(&mut buffer).unwrap();
        result_file.write_all(&buffer[..count]).unwrap();
    }
}
