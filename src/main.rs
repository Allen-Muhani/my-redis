use mini_redis::{Result, client};

#[tokio::main]
async fn main() -> Result<()> {
    // creates a connetion to the mini-redis server.
    let mut client = client::connect("127.0.0.1:6379").await?;


    // Sets the key "hello" with the value "world"
    client.set("hello", "world".into()).await?;


    // Gets the value of the key "hello"
    let result = client.get("hello").await?;
    println!("got value from the server; result={:?}", result);

    Ok(())
}
