use omni_gate_rs::protos::user::user_service_client::UserServiceClient;
use omni_gate_rs::protos::user::SubscribeRequest;
use tonic::transport::Channel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static("http://[::1]:5000").connect().await?;
    let mut client = UserServiceClient::new(channel);

    println!("Testing gRPC streaming subscription...");

    let request = SubscribeRequest {
        user_id: 1,
        interval_seconds: 2,
    };

    let mut stream = client.subscribe_user_updates(request).await?.into_inner();

    println!("Receiving updates:");
    let mut count = 0;
    while let Some(update) = stream.message().await? {
        count += 1;
        println!("[{}] Update: {:?}", count, update);
        if count >= 5 {
            println!("Received 5 updates, stopping...");
            break;
        }
    }

    println!("Test completed!");
    Ok(())
}
