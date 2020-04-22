#[macro_use]
extern crate lazy_static;
use tonic::{transport::Server, Request, Response, Status};
use rsdb::rsdb_server::{Rsdb, RsdbServer};
use rsdb::{SetStringReply, SetStringRequest};
pub mod server_func;
use server_func::{Data, Database, set};
pub mod rsdb {
    tonic::include_proto!("rsdb");
}

#[derive(Default, Clone, Copy)]
pub struct MyRsdb {}

#[tonic::async_trait]
impl Rsdb for MyRsdb {
    async fn set_string(
        &self,
        request: Request<SetStringRequest>,
    ) -> Result<Response<SetStringReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());
        // server_func::push(1);
        let req = request.into_inner();
        let key = req.key.into();
        let val = req.val.into();
        println!("{:?}, {:?}", key, val);
        let res = set(key, val).await.unwrap();
        let reply = rsdb::SetStringReply {
            // status: request.into_inner().key,
            status: 1,
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let rsdb = MyRsdb::default();

    println!("RsdbServer listening on {}", addr);

    Server::builder()
        .add_service(RsdbServer::new(rsdb))
        .serve(addr)
        .await?;

    Ok(())
}