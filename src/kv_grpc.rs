// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

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

const METHOD_KVDB_GET: ::grpcio::Method<super::kv::GetRequest, super::kv::GetResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/crash_kv.KVDB/get",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KVDB_ADD: ::grpcio::Method<super::kv::AddRequest, super::kv::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/crash_kv.KVDB/add",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

pub struct KvdbClient {
    client: ::grpcio::Client,
}

impl KvdbClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        KvdbClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn get_opt(&self, req: &super::kv::GetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kv::GetResponse> {
        self.client.unary_call(&METHOD_KVDB_GET, req, opt)
    }

    pub fn get(&self, req: &super::kv::GetRequest) -> ::grpcio::Result<super::kv::GetResponse> {
        self.get_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_async_opt(&self, req: &super::kv::GetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kv::GetResponse>> {
        self.client.unary_call_async(&METHOD_KVDB_GET, req, opt)
    }

    pub fn get_async(&self, req: &super::kv::GetRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kv::GetResponse>> {
        self.get_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn add_opt(&self, req: &super::kv::AddRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kv::Empty> {
        self.client.unary_call(&METHOD_KVDB_ADD, req, opt)
    }

    pub fn add(&self, req: &super::kv::AddRequest) -> ::grpcio::Result<super::kv::Empty> {
        self.add_opt(req, ::grpcio::CallOption::default())
    }

    pub fn add_async_opt(&self, req: &super::kv::AddRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kv::Empty>> {
        self.client.unary_call_async(&METHOD_KVDB_ADD, req, opt)
    }

    pub fn add_async(&self, req: &super::kv::AddRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kv::Empty>> {
        self.add_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Kvdb {
    fn get(&mut self, ctx: ::grpcio::RpcContext, req: super::kv::GetRequest, sink: ::grpcio::UnarySink<super::kv::GetResponse>);
    fn add(&mut self, ctx: ::grpcio::RpcContext, req: super::kv::AddRequest, sink: ::grpcio::UnarySink<super::kv::Empty>);
}

pub fn create_kvdb<S: Kvdb + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KVDB_GET, move |ctx, req, resp| {
        instance.get(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KVDB_ADD, move |ctx, req, resp| {
        instance.add(ctx, req, resp)
    });
    builder.build()
}
