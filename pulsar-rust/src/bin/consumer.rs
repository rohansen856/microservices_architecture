use std::env;

use futures::TryStreamExt;
use pulsar::{Consumer, DeserializeMessage, Payload, Pulsar, SubType, TokioExecutor};
use serde::{Serialize, Deserialize};
use tokio;

#[derive(Serialize, Deserialize)]
struct TestMessage {
    content: String,
}

impl DeserializeMessage for TestMessage {
    type Output = Result<TestMessage, serde_json::Error>;

    fn deserialize_message(payload: &Payload) -> Self::Output {
        serde_json::from_slice(&payload.data)
    }
}

#[tokio::main]
async fn main() -> Result<(), pulsar::Error> {
    let addr = env::var("PULSAR_ADDRESS")
        .ok()
        .unwrap_or_else(|| "pulsar://127.0.0.1:6650".to_string());
    let topic = env::var("PULSAR_TOPIC")
        .ok()
        .unwrap_or_else(|| "persistent://public/default/test-topic".to_string());

    let pulsar: Pulsar<_> = Pulsar::builder(&addr, TokioExecutor)
        .build()
        .await?;
    
    // Create a consumer
    let mut consumer: Consumer<TestMessage, _> = pulsar
        .consumer()
        .with_topic(&topic)
        .with_subscription("test-subscription")
        .with_subscription_type(SubType::Exclusive)
        .build()
        .await?;
    
    // Receive messages
    while let Some(msg) = consumer.try_next().await? {
        let data = match msg.deserialize() {
            Ok(data) => data,
            Err(e) => {
                println!("{:?}", e);
                break;
            }
        };
        println!("Received: {:?}", data.content);
        consumer.ack(&msg).await?;
    }

    Ok(())
}
