use crate::app::server::Server;

mod app;
mod prompts;
mod users;
mod haiku;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = Server::new().await?;
    server.run().await?;
    Ok(())
}
