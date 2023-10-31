use tonic::{transport::Server, Request, Response, Status};

use api::greeting_service_server::{GreetingService, GreetingServiceServer};
use api::{GreetingRequest, GreetingResponse};

use ::clap::{Parser};

pub mod api {
    tonic::include_proto!("api");
}

#[derive(Debug, Default)]
pub struct Greeting {}

#[tonic::async_trait]
impl GreetingService for Greeting {
    async fn greet(&self, request: Request<GreetingRequest>) -> Result<Response<GreetingResponse>, Status> {
        println!("Got a request: {:?}", request);
        let request = request.into_inner();
        let name = request.name;
        let message = request.message;
        let reply_message = format!("Hello {}! {}", name, message);
        let reply = GreetingResponse {
            message: reply_message,
        };

        Ok(Response::new(reply))
    }
}

#[derive(Parser)]
#[command(author, version)]
#[command(about = "greeting-server - a simple echo microservice", long_about = None)]
struct ServerCli {
    #[arg(short = 's', long = "server", default_value = "127.0.0.1")]
    server: String,
    #[arg(short = 'p', long = "port", default_value = "50052")]
    port: u16,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = ServerCli::parse();
    let addr = format!("{}:{}", cli.server, cli.port).parse()?;
    let greeting = Greeting::default();

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(GreetingServiceServer::new(greeting))
        .serve(addr)
        .await?;

    Ok(())
}