use anyhow::Result;
use async_prost::Frame;
use blake3::Hasher;
use futures::{SinkExt, StreamExt};
use rayon::prelude::*;
use std::thread;
use tokio::{
    net::TcpListener,
    sync::{mpsc, oneshot},
};
use tokio_util::codec::{Framed, LinesCodec};

pub const PREFIX_ZERO: &[u8] = &[0, 0, 0];

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await?;
    println!("Server listening on: {}", addr);
    let (sender, mut receiver) = mpsc::unbounded_channel::<(String, oneshot::Sender<String>)>();

    // 使用thread处理计算密集型任务
    thread::spawn(move || {
        // 读取从tokio task 过来的msg，注意使用的是blocking_recv，不是await
        while let Some((line, reply)) = receiver.blocking_recv() {
            // 计算pow
            let result = match pow(&line) {
                Some((hash, nonce)) => format!("hash: {}, nonce: {}", hash, nonce),
                None => "Not found".to_string(),
            };

            if let Err(e) = reply.send(result) {
                println!("Send Failed");
            }
        }
    });

    // tokio task 处理 IO 密集型任务
    loop {
        let (stream, addr) = listener.accept().await?;
        println!("Accepted: {:?}", addr);
        let sender1 = sender.clone();
        tokio::spawn(async move {
            let framed = Framed::new(stream, LinesCodec::new());
            let (mut w, mut r) = framed.split();
            while let Some(Ok(line)) = r.next().await {
                let (reply, reply_receiver) = oneshot::channel();
                sender1.send((line, reply));

                if let Ok(v) = reply_receiver.await {
                    w.send(format!("Pow calculated: {}", v)).await;
                }
            }
        });
    }
}

// 使用 rayon 并发计算 u32 空间下所有 nonce，直到找到有头 N 个 0 的哈希
pub fn pow(s: &str) -> Option<(String, u32)> {
    let hasher = blake3_base_hash(s.as_bytes());
    let nonce = (0..u32::MAX).into_par_iter().find_any(|n| {
        let hash = blake3_hash(hasher.clone(), n).as_bytes().to_vec();
        &hash[..PREFIX_ZERO.len()] == PREFIX_ZERO
    });
    nonce.map(|n| {
        let hash = blake3_hash(hasher, &n).to_hex().to_string();
        (hash, n)
    })
}

fn blake3_hash(mut hasher: Hasher, nonce: &u32) -> blake3::Hash {
    hasher.update(&nonce.to_be_bytes()[..]);
    hasher.finalize()
}
fn blake3_base_hash(data: &[u8]) -> Hasher {
    let mut hasher = Hasher::new();
    hasher.update(data);
    hasher
}
