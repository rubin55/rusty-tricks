use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Open a connection to mini-redis.
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set the key 'hello' to the value 'world'.
    client.set("hello", "world".into()).await?;

    // Get value for key 'hello'.
    let result = client.get("hello").await?;

    println!("Got value from the server: {:?}", result);

    Ok(())
}
