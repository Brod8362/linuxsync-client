// This file is generated by rust-protobuf 2.14.0. Do not edit
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
//! Generated file from `auth.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_14_0;

#[derive(PartialEq,Clone,Default)]
pub struct ClientDetails {
    // message fields
    hostname: ::protobuf::SingularField<::std::string::String>,
    pubkey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ClientDetails {
    fn default() -> &'a ClientDetails {
        <ClientDetails as ::protobuf::Message>::default_instance()
    }
}

impl ClientDetails {
    pub fn new() -> ClientDetails {
        ::std::default::Default::default()
    }

    // required string hostname = 1;


    pub fn get_hostname(&self) -> &str {
        match self.hostname.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
    pub fn clear_hostname(&mut self) {
        self.hostname.clear();
    }

    pub fn has_hostname(&self) -> bool {
        self.hostname.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hostname(&mut self, v: ::std::string::String) {
        self.hostname = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hostname(&mut self) -> &mut ::std::string::String {
        if self.hostname.is_none() {
            self.hostname.set_default();
        }
        self.hostname.as_mut().unwrap()
    }

    // Take field
    pub fn take_hostname(&mut self) -> ::std::string::String {
        self.hostname.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // optional bytes pubkey = 2;


    pub fn get_pubkey(&self) -> &[u8] {
        match self.pubkey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
    pub fn clear_pubkey(&mut self) {
        self.pubkey.clear();
    }

    pub fn has_pubkey(&self) -> bool {
        self.pubkey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pubkey(&mut self, v: ::std::vec::Vec<u8>) {
        self.pubkey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pubkey(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.pubkey.is_none() {
            self.pubkey.set_default();
        }
        self.pubkey.as_mut().unwrap()
    }

    // Take field
    pub fn take_pubkey(&mut self) -> ::std::vec::Vec<u8> {
        self.pubkey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for ClientDetails {
    fn is_initialized(&self) -> bool {
        if self.hostname.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.hostname)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.pubkey)?;
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
        if let Some(ref v) = self.hostname.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.pubkey.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.hostname.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.pubkey.as_ref() {
            os.write_bytes(2, &v)?;
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

    fn new() -> ClientDetails {
        ClientDetails::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "hostname",
                    |m: &ClientDetails| { &m.hostname },
                    |m: &mut ClientDetails| { &mut m.hostname },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "pubkey",
                    |m: &ClientDetails| { &m.pubkey },
                    |m: &mut ClientDetails| { &mut m.pubkey },
                ));
                ::protobuf::reflect::MessageDescriptor::new_pb_name::<ClientDetails>(
                    "ClientDetails",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static ClientDetails {
        static mut instance: ::protobuf::lazy::Lazy<ClientDetails> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            instance.get(ClientDetails::new)
        }
    }
}

impl ::protobuf::Clear for ClientDetails {
    fn clear(&mut self) {
        self.hostname.clear();
        self.pubkey.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClientDetails {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClientDetails {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\nauth.proto\x12\tlinuxsync\"C\n\rClientDetails\x12\x1a\n\x08hostname\
    \x18\x01\x20\x02(\tR\x08hostname\x12\x16\n\x06pubkey\x18\x02\x20\x01(\
    \x0cR\x06pubkeyB2\n\x20pw.byakuren.linuxsync.encryptionB\x0eAuthenticati\
    on\
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
