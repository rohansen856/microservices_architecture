use redis::Client;
use futures::StreamExt;

#[tokio::main]
async fn main() -> redis::RedisResult<()> {
    let client = Client::open("redis://127.0.0.1/")?;

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
