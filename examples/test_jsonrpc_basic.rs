use jsonrpsee::core::client::ClientT;
use jsonrpsee::ws_client::WsClientBuilder;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "ws://127.0.0.1:4000";
    let client = WsClientBuilder::default().build(url).await?;

    println!("Testing JSON-RPC methods...");

    let result: serde_json::Value = client.request("get_user_info", ((),)).await?;
    println!("get_user_info result: {}", result);

    let params = json!({
        "name": "Test User",
        "email": "test@example.com",
        "age": 25
    });
    let result: serde_json::Value = client.request("update_user_info", (params,)).await?;
    println!("update_user_info result: {}", result);

    let params = json!({
        "username": "testuser",
        "password": "testpass"
    });
    let result: serde_json::Value = client.request("verify_credentials", (params,)).await?;
    println!("verify_credentials result: {}", result);

    println!("Test completed!");
    Ok(())
}
