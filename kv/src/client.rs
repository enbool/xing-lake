use crate::network::ProstClientStream;
use crate::CommandRequest;
use anyhow::Result;
use tokio::net::TcpStream;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:8080";

    let stream = TcpStream::connect(addr).await?;
    let mut client = ProstClientStream::new(stream);

    // 生成一个 HSET 命令
    let cmd = CommandRequest::new_hset("table1", "hello", "world".to_string().into());
    let data = client.execute(cmd).await?;

    info!("Got response: {:?}", data);

    Ok(())
}
