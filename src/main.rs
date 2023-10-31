use api::greeting_service_client::GreetingServiceClient;
use api::GreetingRequest;
use ::clap::{Parser};

pub mod api {
    tonic::include_proto!("api");
}

#[derive(Parser)]
#[command(author, version)]
#[command(about = "greet - a simple CLI to send messages to a server", long_about = None)]
struct ClientCli {
    #[arg(short = 's', long = "server", default_value = "127.0.0.1")]
    server: String,
    #[arg(short = 'p', long = "port", default_value = "50052")]
    port: u16,
    /// The message to send
    name: String,
    message: String,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = ClientCli::parse();

    let mut client = GreetingServiceClient::connect(format!("http://{}:{}", cli.server, cli.port)).await?;

    let request = tonic::Request::new(GreetingRequest {
        name: cli.name,
        message: cli.message,
    });

    let response = client.greet(request).await?;

    println!("RESPONSE={:?}", response.into_inner().message);

    Ok(())
}