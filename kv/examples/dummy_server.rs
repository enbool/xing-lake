use anyhow::Result;
use async_prost::AsyncProstStream;
use futures::{SinkExt, StreamExt};
use kv::{CommandRequest, CommandResponse, KvError};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:9527";

    let listener = TcpListener::bind(addr).await?;
    info!("Server listening on {}", addr);

    loop {
        let (stream, addr) = listener.accept().await?;
        info!("client {:?} connected", addr);

        tokio::spawn(async move {
            let mut stream =
                AsyncProstStream::<_, CommandRequest, CommandResponse, _>::from(stream).for_async();

            while let Some(Ok(msg)) = stream.next().await {
                info!("Got command: {:?}", msg);
                let resp = CommandResponse::from(KvError::NotFound("Nil".into(), "Nil".into()));
                stream.send(resp).await.unwrap();
            }
            info!("client {:?} disconnected", addr);
        });
    }
}
