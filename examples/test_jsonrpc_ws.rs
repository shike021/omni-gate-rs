use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "ws://127.0.0.1:4000";
    let request = url.into_client_request()?;

    println!("Connecting to JSON-RPC WebSocket server...");
    let (ws_stream, _) = connect_async(request).await?;
    let (mut write, mut read) = ws_stream.split();

    println!("Subscribing to user updates...");
    let subscribe_msg = r#"{"jsonrpc":"2.0","method":"subscribe_user_updates","params":{"user_id":1,"interval_seconds":2},"id":1}"#;
    write
        .send(tokio_tungstenite::tungstenite::Message::Text(
            subscribe_msg.into(),
        ))
        .await?;

    println!("Receiving updates:");
    let mut count = 0;
    while let Some(msg) = read.next().await {
        let msg = msg?;
        if let tokio_tungstenite::tungstenite::Message::Text(text) = msg {
            count += 1;
            println!("[{}] Update: {}", count, text);
            if count >= 5 {
                println!("Received 5 updates, stopping...");
                break;
            }
        }
    }

    println!("Test completed!");
    Ok(())
}
