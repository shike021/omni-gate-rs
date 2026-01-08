use jsonrpsee::core::client::SubscriptionClientT;
use jsonrpsee::ws_client::WsClientBuilder;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "ws://127.0.0.1:4000";
    let client = WsClientBuilder::default().build(url).await?;

    println!("Testing JSON-RPC subscription...");

    let params = json!({
        "user_id": 1,
        "interval_seconds": 2
    });

    let mut sub: jsonrpsee::core::client::Subscription<serde_json::Value> = client
        .subscribe(
            "subscribe_user_updates",
            (params,),
            "unsubscribe_user_updates",
        )
        .await?;

    println!("Receiving updates:");
    let mut count = 0;
    while let Some(update) = sub.next().await {
        count += 1;
        match update {
            Ok(val) => println!("[{}] Update: {}", count, val),
            Err(e) => println!("[{}] Error: {}", count, e),
        }
        if count >= 5 {
            println!("Received 5 updates, stopping...");
            break;
        }
    }

    println!("Test completed!");
    Ok(())
}
