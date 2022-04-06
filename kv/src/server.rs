use crate::network::ProstServerStream;
use crate::{MemTable, Service};
use anyhow::Result;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let service = Service::new(MemTable::new());
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await?;
    info!("Server started on: {}", addr);

    while let (stream, addr) = listener.accept().await? {
        info!("Client:{} connected", addr);
        let server = ProstServerStream::new(stream, service.clone());
        tokio::spawn(async move { server.process().await });
    }

    Ok(())
}
