// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_FILE_SERVICE_DOWNLOAD: ::grpcio::Method<super::rpc::DownloadRequest, super::rpc::DownloadResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/FileService/download",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_FILE_SERVICE_UPLOAD: ::grpcio::Method<super::rpc::UploadRequest, super::rpc::UploadResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/FileService/upload",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct FileServiceClient {
    client: ::grpcio::Client,
}

impl FileServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        FileServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn download_opt(&self, req: &super::rpc::DownloadRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::DownloadResponse> {
        self.client.unary_call(&METHOD_FILE_SERVICE_DOWNLOAD, req, opt)
    }

    pub fn download(&self, req: &super::rpc::DownloadRequest) -> ::grpcio::Result<super::rpc::DownloadResponse> {
        self.download_opt(req, ::grpcio::CallOption::default())
    }

    pub fn download_async_opt(&self, req: &super::rpc::DownloadRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::DownloadResponse>> {
        self.client.unary_call_async(&METHOD_FILE_SERVICE_DOWNLOAD, req, opt)
    }

    pub fn download_async(&self, req: &super::rpc::DownloadRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::DownloadResponse>> {
        self.download_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn upload_opt(&self, req: &super::rpc::UploadRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::UploadResponse> {
        self.client.unary_call(&METHOD_FILE_SERVICE_UPLOAD, req, opt)
    }

    pub fn upload(&self, req: &super::rpc::UploadRequest) -> ::grpcio::Result<super::rpc::UploadResponse> {
        self.upload_opt(req, ::grpcio::CallOption::default())
    }

    pub fn upload_async_opt(&self, req: &super::rpc::UploadRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::UploadResponse>> {
        self.client.unary_call_async(&METHOD_FILE_SERVICE_UPLOAD, req, opt)
    }

    pub fn upload_async(&self, req: &super::rpc::UploadRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::UploadResponse>> {
        self.upload_async_opt(req, ::grpcio::CallOption::default())
    }
    // pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output=()> + Send + 'static {
    //     self.client.spawn(f)
    // }
}

pub trait FileService {
    fn download(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::DownloadRequest, sink: ::grpcio::UnarySink<super::rpc::DownloadResponse>);
    fn upload(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::UploadRequest, sink: ::grpcio::UnarySink<super::rpc::UploadResponse>);
}

pub fn create_file_service<S: FileService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_FILE_SERVICE_DOWNLOAD, move |ctx, req, resp| {
        instance.download(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_FILE_SERVICE_UPLOAD, move |ctx, req, resp| {
        instance.upload(ctx, req, resp)
    });
    builder.build()
}
