use tonic::service::interceptor::InterceptedService;
use tonic::{codec::CompressionEncoding, transport::Server, Request, Response, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloRequest, HelloResponse};

use hello_world::lover_server::{Lover, LoverServer};
use hello_world::{LoveRequest, LoveResponse};

pub mod hello_world {
    tonic::include_proto!("helloworld"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        let response = hello_world::HelloResponse {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(response))
    }
}

#[derive(Debug, Default)]
pub struct MyLover {}

#[tonic::async_trait]
impl Lover for MyLover {
    async fn say_love(
        &self,
        request: Request<LoveRequest>,
    ) -> Result<Response<LoveResponse>, Status> {
        Ok(Response::new(LoveResponse {
            message: format!("I love {}!", request.into_inner().name),
        }))
    }
}

static AUTH_TOKENS: [&str; 2] = ["S3CR37-70K3N-1", "S3CR37-70K3N-2"];
fn check_auth(req: Request<()>) -> Result<Request<()>, Status> {
    println!("Got a request: {:?}", req);
    let unauthorized_status = Status::unauthenticated("Invalid auth token");

    // Get "authorization" metadata
    let Some(auth_metadata_value) = req.metadata().get("Authorization") else { 
        return Err(unauthorized_status);
    };

    // Convert the metadata value to a string
    let Ok(auth_metadata_value) = auth_metadata_value.to_str() else {
        return Err(unauthorized_status);
    };

    // Excluding the "Bearer " prefix
    let Some(token) = auth_metadata_value.get(7..) else {
        return Err(unauthorized_status);
    };

    // Check if the token is valid
    if AUTH_TOKENS.contains(&token) {
        Ok(req)
    } else {
        Err(unauthorized_status)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    Server::builder()
        .add_service(InterceptedService::new(
            GreeterServer::new(MyGreeter::default())
                .accept_compressed(CompressionEncoding::Gzip)
                .send_compressed(CompressionEncoding::Gzip),
            check_auth,
        ))
        .add_service(InterceptedService::new(
            LoverServer::new(MyLover::default())
                .accept_compressed(CompressionEncoding::Gzip)
                .send_compressed(CompressionEncoding::Gzip),
            check_auth,
        ))
        .serve(addr)
        .await?;

    Ok(())
}

// cd ~/Workspace/grpctest/
// grpcurl -plaintext -import-path ./grpctest-proto -proto helloworld.proto -d '{"name": "Yoran"}' '[::1]:50051' helloworld.Greeter/SayHello
// grpcurl -plaintext -import-path ./grpctest-proto -proto helloworld.proto -d '{"name": "Jiaxi"}' '[::1]:50051' helloworld.Lover/SayLove
