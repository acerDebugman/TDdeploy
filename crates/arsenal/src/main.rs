use arsenal::serve;

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    println!("Hello, world!");
    // serve::start_server1().await?;
    serve::start_server2().await?;
    Ok(())
}
