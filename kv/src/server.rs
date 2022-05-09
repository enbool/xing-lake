use anyhow::Result;
use kv::{MemTable, ProstServerStream, Service};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let service = Service::new(MemTable::new());
    let addr = "127.0.0.1:9527";
    let listener = TcpListener::bind(addr).await?;
    println!("Server started on: {}", addr);

    while let (stream, addr) = listener.accept().await? {
        println!("Client:{} connected", addr);
        let server = ProstServerStream::new(stream, service.clone());
        tokio::spawn(async move { server.process().await });
    }

    Ok(())
}
