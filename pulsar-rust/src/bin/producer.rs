use std::env;

use pulsar::{producer, Error, Pulsar, SerializeMessage, TokioExecutor, Error as PulsarError};
use serde::{Serialize, Deserialize};
use tokio;

#[derive(Serialize, Deserialize)]
struct TestMessage {
    content: String,
}

impl SerializeMessage for TestMessage {
    fn serialize_message(input: Self) -> Result<producer::Message, PulsarError> {
        let payload = serde_json::to_vec(&input).map_err(|e| PulsarError::Custom(e.to_string()))?;
        Ok(producer::Message {
            payload,
            ..Default::default()
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let addr = env::var("PULSAR_ADDRESS")
        .ok()
        .unwrap_or_else(|| "pulsar://127.0.0.1:6650".to_string());
    let topic = env::var("PULSAR_TOPIC")
        .ok()
        .unwrap_or_else(|| "persistent://public/default/test-topic".to_string());

    let pulsar: Pulsar<_> = Pulsar::builder(&addr, TokioExecutor)
        .build()
        .await?;
    
    let mut producer = pulsar
        .producer()
        .with_topic(&topic)
        .with_name("test-producer")
        .build()
        .await?;
    
    let message = TestMessage {
        content: "Hello, Pulsar!".to_string(),
    };
    
    producer.send_non_blocking(message).await?;
    println!("Message sent!");

    Ok(())
}
