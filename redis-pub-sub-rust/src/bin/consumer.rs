use redis::Client;
use futures::StreamExt;
use std::env;

#[tokio::main]
async fn main() -> redis::RedisResult<()> {
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());

    let client = Client::open(redis_url)?;

    let subscriber_client = client.clone();
    if let Err(err) = subscribe_to_channel(subscriber_client).await {
        eprintln!("Error in subscriber: {}", err);
    }

    Ok(())
}

async fn subscribe_to_channel(client: Client) -> redis::RedisResult<()> {
    let connection = client.get_async_pubsub().await?;
    let mut pubsub = connection;

    let channel = "my_channel";
    pubsub.subscribe(channel).await?;
    println!("Subscribed to channel: {}", channel);

    let mut message_stream = pubsub.on_message();

    while let Some(msg) = message_stream.next().await {
        let payload: String = msg.get_payload()?;
        println!("Received: {}", payload);
    }

    Ok(())
}
