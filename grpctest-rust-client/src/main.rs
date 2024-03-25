use tonic::codec::CompressionEncoding;
use tonic::metadata::MetadataValue;
use tonic::Request;
use tonic::service::interceptor::InterceptedService;
use tonic::transport::Endpoint;

use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;

use hello_world::lover_client::LoverClient;
use hello_world::LoveRequest;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}


static AUTH_TOKEN: &str = "S3CR37-70K3N-1";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Endpoint::from_static("https://[::1]:50051").connect_lazy();
    let auth_token: MetadataValue<_> = format!("Bearer {}", AUTH_TOKEN).parse()?; 

    let interceptor = InterceptedService::new(channel, |mut req: Request<()>| {
        req.metadata_mut().insert("authorization", auth_token.clone());
        Ok(req)
    });

    let mut greeter_client = GreeterClient::new(interceptor.clone())
        .accept_compressed(CompressionEncoding::Gzip)
        .send_compressed(CompressionEncoding::Gzip);
    let mut lover_client = LoverClient::new(interceptor)
        .accept_compressed(CompressionEncoding::Gzip)
        .send_compressed(CompressionEncoding::Gzip);

    // Ask for name
    let mut my_name = String::new();
    println!("Who are you?");
    std::io::stdin().read_line(&mut my_name)?;
    let my_name = my_name.trim();

    let hello_response = greeter_client
        .say_hello(tonic::Request::new(HelloRequest {
            name: my_name.to_string(),
        }))
        .await?;
    println!("{}", hello_response.get_ref().message);

    // Ask for lover'sname
    let mut lover_name = String::new();
    println!("Who's your lover?");
    std::io::stdin().read_line(&mut lover_name)?;
    let lover_name = lover_name.trim();

    let love_response = lover_client
        .say_love(tonic::Request::new(LoveRequest {
            name: lover_name.to_string(),
        }))
        .await?;
    println!("{}", love_response.get_ref().message);

    Ok(())
}
