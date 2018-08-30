extern crate futures;
extern crate protobuf;
extern crate grpcio;
extern crate grpcio_proto;
extern crate parking_lot;
extern crate rocksdb;
#[cfg(test)] extern crate tempfile;

mod kv;
mod kv_grpc;

use std::sync::Arc;
use futures::Future;
use grpcio::*;
use parking_lot::RwLock;
use rocksdb::{Writable, DB};

use kv::*;
use kv_grpc::*;

#[derive(Clone)]
struct KVDBService {
    db: Arc<RwLock<DB>>,
}

impl Kvdb for KVDBService {
    fn get(&self, ctx: RpcContext, req: GetRequest, sink: UnarySink<GetResponse>) {
        let key = req.get_key();
        let db = self.db.clone();
        let value = db
            .read()
            .get(key.as_bytes())
            .ok()
            .and_then(|v| v.map(|x| x.to_utf8().unwrap().to_string()))
            .unwrap_or_else(String::new);
        let mut resp = GetResponse::new();
        resp.set_key(key.to_string());
        resp.set_value(value);
        let f = sink
            .success(resp)
            .map_err(|e| println!("Failed to get: {:?}", e));
        ctx.spawn(f)
    }

    fn add(&self, ctx: RpcContext, req: AddRequest, sink: UnarySink<Empty>) {
        let key = req.get_key();
        let value = req.get_value();
        let db = self.db.clone();
        db.write().put(key.as_bytes(), value.as_bytes()).unwrap();
        let f = sink
            .success(Empty::new())
            .map_err(|e| println!("Failed to add: {:?}", e));
        ctx.spawn(f)
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use std::sync::Arc;
    use tempfile::tempdir;
    use futures::Future;
    use grpcio::{Environment, ChannelBuilder, Server, ServerBuilder};
    use parking_lot::RwLock;
    use rocksdb::DB;

    use kv::*;
    use kv_grpc::*;
    use super::KVDBService;

    fn start_server() -> Server {
        let dir = tempdir().unwrap();
        let data_path = dir.path().to_str().unwrap();
        let db = DB::open_default(data_path).unwrap();
        let instance = KVDBService {
            db: Arc::new(RwLock::new(db)),
        };
        let service = create_kvdb(instance);
        let env = Arc::new(Environment::new(2));
        let mut server = ServerBuilder::new(env)
            .register_service(service)
            .bind("127.0.0.1", 0)
            .build()
            .unwrap();
        server.start();
        server
    }

    #[test]
    fn test_grpc_apis() {
        let server = start_server();
        let (_, port) = server.bind_addrs()[0];

        let env = Arc::new(Environment::new(1));
        let ch = ChannelBuilder::new(env).connect(&format!("127.0.0.1:{}", port));
        let client = KvdbClient::new(ch);

        let mut req = AddRequest::new();
        req.set_key("test".into());
        req.set_value("test".into());
        let res = client.add_async(&req).unwrap();
        res.wait().unwrap();

        let mut req = GetRequest::new();
        req.set_key("test".into());
        let res = client.get_async(&req).unwrap();
        let res = res.wait().unwrap();
        assert_eq!(res.get_key(), "test");
        assert_eq!(res.get_value(), "test");
    }
}
