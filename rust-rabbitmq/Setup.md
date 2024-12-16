### Setup the rabbitmq image and server
## Start the server
```sh
sudo docker compose up
sudo docker exec rabbitmq rabbitmq-plugins enable rabbitmq_stream
```
## Start the producer
```sh
cargo run --bin send
```
## Start the consumer
```sh
cargo run --bin receive
```