// This file is generated by rust-protobuf 2.12.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
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
//! Generated file from `rpc/rpc.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_12_0;

#[derive(PartialEq,Clone,Default)]
pub struct DownloadRequest {
    // message fields
    pub Filename: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a DownloadRequest {
    fn default() -> &'a DownloadRequest {
        <DownloadRequest as ::protobuf::Message>::default_instance()
    }
}

impl DownloadRequest {
    pub fn new() -> DownloadRequest {
        ::std::default::Default::default()
    }

    // string Filename = 1;


    pub fn get_Filename(&self) -> &str {
        &self.Filename
    }
    pub fn clear_Filename(&mut self) {
        self.Filename.clear();
    }

    // Param is passed by value, moved
    pub fn set_Filename(&mut self, v: ::std::string::String) {
        self.Filename = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_Filename(&mut self) -> &mut ::std::string::String {
        &mut self.Filename
    }

    // Take field
    pub fn take_Filename(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.Filename, ::std::string::String::new())
    }
}

impl ::protobuf::Message for DownloadRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.Filename)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.Filename.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.Filename);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.Filename.is_empty() {
            os.write_string(1, &self.Filename)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> DownloadRequest {
        DownloadRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "Filename",
                    |m: &DownloadRequest| { &m.Filename },
                    |m: &mut DownloadRequest| { &mut m.Filename },
                ));
                ::protobuf::reflect::MessageDescriptor::new_pb_name::<DownloadRequest>(
                    "DownloadRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static DownloadRequest {
        static mut instance: ::protobuf::lazy::Lazy<DownloadRequest> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            instance.get(DownloadRequest::new)
        }
    }
}

impl ::protobuf::Clear for DownloadRequest {
    fn clear(&mut self) {
        self.Filename.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DownloadRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DownloadRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DownloadResponse {
    // message fields
    pub data: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a DownloadResponse {
    fn default() -> &'a DownloadResponse {
        <DownloadResponse as ::protobuf::Message>::default_instance()
    }
}

impl DownloadResponse {
    pub fn new() -> DownloadResponse {
        ::std::default::Default::default()
    }

    // bytes data = 1;


    pub fn get_data(&self) -> &[u8] {
        &self.data
    }
    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for DownloadResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.data)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.data);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.data.is_empty() {
            os.write_bytes(1, &self.data)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> DownloadResponse {
        DownloadResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    |m: &DownloadResponse| { &m.data },
                    |m: &mut DownloadResponse| { &mut m.data },
                ));
                ::protobuf::reflect::MessageDescriptor::new_pb_name::<DownloadResponse>(
                    "DownloadResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static DownloadResponse {
        static mut instance: ::protobuf::lazy::Lazy<DownloadResponse> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            instance.get(DownloadResponse::new)
        }
    }
}

impl ::protobuf::Clear for DownloadResponse {
    fn clear(&mut self) {
        self.data.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DownloadResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DownloadResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UploadRequest {
    // message fields
    pub Filename: ::std::string::String,
    pub data: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a UploadRequest {
    fn default() -> &'a UploadRequest {
        <UploadRequest as ::protobuf::Message>::default_instance()
    }
}

impl UploadRequest {
    pub fn new() -> UploadRequest {
        ::std::default::Default::default()
    }

    // string Filename = 1;


    pub fn get_Filename(&self) -> &str {
        &self.Filename
    }
    pub fn clear_Filename(&mut self) {
        self.Filename.clear();
    }

    // Param is passed by value, moved
    pub fn set_Filename(&mut self, v: ::std::string::String) {
        self.Filename = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_Filename(&mut self) -> &mut ::std::string::String {
        &mut self.Filename
    }

    // Take field
    pub fn take_Filename(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.Filename, ::std::string::String::new())
    }

    // bytes data = 2;


    pub fn get_data(&self) -> &[u8] {
        &self.data
    }
    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for UploadRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.Filename)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.data)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.Filename.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.Filename);
        }
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.data);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.Filename.is_empty() {
            os.write_string(1, &self.Filename)?;
        }
        if !self.data.is_empty() {
            os.write_bytes(2, &self.data)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> UploadRequest {
        UploadRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "Filename",
                    |m: &UploadRequest| { &m.Filename },
                    |m: &mut UploadRequest| { &mut m.Filename },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    |m: &UploadRequest| { &m.data },
                    |m: &mut UploadRequest| { &mut m.data },
                ));
                ::protobuf::reflect::MessageDescriptor::new_pb_name::<UploadRequest>(
                    "UploadRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static UploadRequest {
        static mut instance: ::protobuf::lazy::Lazy<UploadRequest> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            instance.get(UploadRequest::new)
        }
    }
}

impl ::protobuf::Clear for UploadRequest {
    fn clear(&mut self) {
        self.Filename.clear();
        self.data.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UploadRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UploadRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UploadResponse {
    // message fields
    pub success: bool,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a UploadResponse {
    fn default() -> &'a UploadResponse {
        <UploadResponse as ::protobuf::Message>::default_instance()
    }
}

impl UploadResponse {
    pub fn new() -> UploadResponse {
        ::std::default::Default::default()
    }

    // bool success = 1;


    pub fn get_success(&self) -> bool {
        self.success
    }
    pub fn clear_success(&mut self) {
        self.success = false;
    }

    // Param is passed by value, moved
    pub fn set_success(&mut self, v: bool) {
        self.success = v;
    }
}

impl ::protobuf::Message for UploadResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.success = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.success != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.success != false {
            os.write_bool(1, self.success)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> UploadResponse {
        UploadResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "success",
                    |m: &UploadResponse| { &m.success },
                    |m: &mut UploadResponse| { &mut m.success },
                ));
                ::protobuf::reflect::MessageDescriptor::new_pb_name::<UploadResponse>(
                    "UploadResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static UploadResponse {
        static mut instance: ::protobuf::lazy::Lazy<UploadResponse> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            instance.get(UploadResponse::new)
        }
    }
}

impl ::protobuf::Clear for UploadResponse {
    fn clear(&mut self) {
        self.success = false;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UploadResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UploadResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rrpc/rpc.proto\"-\n\x0fDownloadRequest\x12\x1a\n\x08Filename\x18\x01\
    \x20\x01(\tR\x08Filename\"&\n\x10DownloadResponse\x12\x12\n\x04data\x18\
    \x01\x20\x01(\x0cR\x04data\"?\n\rUploadRequest\x12\x1a\n\x08Filename\x18\
    \x01\x20\x01(\tR\x08Filename\x12\x12\n\x04data\x18\x02\x20\x01(\x0cR\x04\
    data\"*\n\x0eUploadResponse\x12\x18\n\x07success\x18\x01\x20\x01(\x08R\
    \x07success2m\n\x0bFileService\x121\n\x08download\x12\x10.DownloadReques\
    t\x1a\x11.DownloadResponse\"\0\x12+\n\x06upload\x12\x0e.UploadRequest\
    \x1a\x0f.UploadResponse\"\0J\xd8\x04\n\x06\x12\x04\0\0\x14\x01\n\x08\n\
    \x01\x0c\x12\x03\0\0\x12\n\n\n\x02\x04\0\x12\x04\x02\0\x04\x01\n\n\n\x03\
    \x04\0\x01\x12\x03\x02\x08\x17\n\x0b\n\x04\x04\0\x02\0\x12\x03\x03\x04\
    \x18\n\r\n\x05\x04\0\x02\0\x04\x12\x04\x03\x04\x02\x19\n\x0c\n\x05\x04\0\
    \x02\0\x05\x12\x03\x03\x04\n\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x03\x0b\
    \x13\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x03\x16\x17\n\n\n\x02\x04\x01\
    \x12\x04\x05\0\x07\x01\n\n\n\x03\x04\x01\x01\x12\x03\x05\x08\x18\n\x0b\n\
    \x04\x04\x01\x02\0\x12\x03\x06\x04\x13\n\r\n\x05\x04\x01\x02\0\x04\x12\
    \x04\x06\x04\x05\x1a\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\x06\x04\t\n\
    \x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x06\n\x0e\n\x0c\n\x05\x04\x01\x02\0\
    \x03\x12\x03\x06\x11\x12\n\n\n\x02\x04\x02\x12\x04\x08\0\x0b\x01\n\n\n\
    \x03\x04\x02\x01\x12\x03\x08\x08\x15\n\x0b\n\x04\x04\x02\x02\0\x12\x03\t\
    \x04\x18\n\r\n\x05\x04\x02\x02\0\x04\x12\x04\t\x04\x08\x17\n\x0c\n\x05\
    \x04\x02\x02\0\x05\x12\x03\t\x04\n\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\
    \t\x0b\x13\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\t\x16\x17\n\x0b\n\x04\
    \x04\x02\x02\x01\x12\x03\n\x04\x13\n\r\n\x05\x04\x02\x02\x01\x04\x12\x04\
    \n\x04\t\x18\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\x03\n\x04\t\n\x0c\n\x05\
    \x04\x02\x02\x01\x01\x12\x03\n\n\x0e\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\
    \x03\n\x11\x12\n\n\n\x02\x04\x03\x12\x04\x0c\0\x0e\x01\n\n\n\x03\x04\x03\
    \x01\x12\x03\x0c\x08\x16\n\x0b\n\x04\x04\x03\x02\0\x12\x03\r\x04\x15\n\r\
    \n\x05\x04\x03\x02\0\x04\x12\x04\r\x04\x0c\x18\n\x0c\n\x05\x04\x03\x02\0\
    \x05\x12\x03\r\x04\x08\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03\r\t\x10\n\
    \x0c\n\x05\x04\x03\x02\0\x03\x12\x03\r\x13\x14\n\n\n\x02\x06\0\x12\x04\
    \x0f\0\x14\x01\n\n\n\x03\x06\0\x01\x12\x03\x0f\x08\x13\n\x0c\n\x04\x06\0\
    \x02\0\x12\x04\x10\x04\x11\x05\n\x0c\n\x05\x06\0\x02\0\x01\x12\x03\x10\
    \x08\x10\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03\x10\x12!\n\x0c\n\x05\x06\0\
    \x02\0\x03\x12\x03\x10,<\n\x0c\n\x04\x06\0\x02\x01\x12\x04\x12\x04\x13\
    \x05\n\x0c\n\x05\x06\0\x02\x01\x01\x12\x03\x12\x08\x0e\n\x0c\n\x05\x06\0\
    \x02\x01\x02\x12\x03\x12\x10\x1d\n\x0c\n\x05\x06\0\x02\x01\x03\x12\x03\
    \x12(6b\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
