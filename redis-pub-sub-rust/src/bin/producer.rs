use redis::{AsyncCommands, Client};
use std::env;

#[tokio::main]
async fn main() -> redis::RedisResult<()> {
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());

    let client = Client::open(redis_url)?;

    let publisher_client = client.clone();
    publish_messages(publisher_client, "my_channel").await?;

    Ok(())
}

async fn publish_messages(client: Client, channel: &str) -> redis::RedisResult<()> {
    let mut connection = client.get_multiplexed_async_connection().await?;

    for i in 1..=5 {
        let message = format!("Message {}", i);
        connection.publish(channel, &message).await?;
        println!("Published: {}", message);
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    Ok(())
}
