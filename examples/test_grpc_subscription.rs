use tokio_stream::StreamExt;
use tonic::Request;

mod protos {
    tonic::include_proto!("user");
}

use protos::user_service_client::UserServiceClient;
use protos::SubscribeRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = UserServiceClient::connect("http://[::1]:5000").await?;

    println!("Subscribing to user updates via gRPC streaming...");
    let request = Request::new(SubscribeRequest {
        user_id: 1,
        interval_seconds: 2,
    });

    let mut stream = client.subscribe_user_updates(request).await?.into_inner();

    println!("Receiving updates:");
    let mut count = 0;
    while let Some(result) = stream.next().await {
        match result {
            Ok(update) => {
                count += 1;
                println!("[{}] Update: {:?}", count, update);
                if count >= 5 {
                    println!("Received 5 updates, stopping...");
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error receiving update: {}", e);
                break;
            }
        }
    }

    println!("Test completed!");
    Ok(())
}
