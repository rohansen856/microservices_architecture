use std::env;

use rabbitmq_stream_client::error::StreamCreateError;
use rabbitmq_stream_client::types::{ByteCapacity, Message, ResponseCode};
use rabbitmq_stream_client::Environment;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let username = env::var("RABBITMQ_USERNAME").unwrap_or_else(|_| "admin".to_string());
    let password = env::var("RABBITMQ_PASSWORD").unwrap_or_else(|_| "admin".to_string());
    
    let environment = Environment::builder()
        .username(&username)
        .password(&password)
        .build()
        .await?;

    let stream = "hello-rust-stream";
    let create_response = environment
        .stream_creator()
        .max_length(ByteCapacity::GB(5))
        .create(stream)
        .await;

    if let Err(e) = create_response {
        if let StreamCreateError::Create { stream, status } = e {
            match status {
                // we can ignore this error because the stream already exists
                ResponseCode::StreamAlreadyExists => {}
                err => {
                    println!("Error creating stream: {:?} {:?}", stream, err);
                }
            }
        }
    }

    let producer = environment.producer().build(stream).await?;

    producer
        .send_with_confirm(Message::builder().body("Hello, World!").build())
        .await?;
    println!("Sent message to stream: {}", stream);
    producer.close().await?;
    Ok(())
}