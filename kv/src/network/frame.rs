
use std::io::{Read, Write};

use crate::{CommandRequest, CommandResponse, KvError};
use bytes::{Buf, BufMut, BytesMut};
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use prost::Message;
use tokio::io::{AsyncRead, AsyncReadExt};
use tracing::debug;

// 长度用4个字节表示，首位表示是否压缩
pub const LEN_LEN: usize = 4;
// 31位，最大长度位2G
const MAX_LEN: usize = 2 * 1024 * 1024 * 1024;
// 长度超过限制后就压缩
const COMPRESSION_LIMIT: usize = 1436;
// 首位表示是否压缩
const COMPRESSION_BIT: usize = 1 << 31;

pub trait FrameCoder
where
    Self: Message + Sized + Default,
{
    // 把一个Message encode成一个Frame
    fn encode_frame(&self, buf: &mut BytesMut) -> Result<(), KvError> {
        let size = self.encoded_len();
        if size >= MAX_LEN {
            return Err(KvError::FrameError);
        }
        // 先写入长度
        buf.put_u32(size as _);
        if size > COMPRESSION_LIMIT {
            let mut buf1 = Vec::with_capacity(size);
            self.encode(&mut buf1)?;

            // BytesMut 支持逻辑上的spilt后再次unsplit
            let payload = buf.split_off(LEN_LEN);
            buf.clear();
            // 处理 gzip 压缩，具体可以参考 flate2 文档
            let mut encoder = GzEncoder::new(payload.writer(), Compression::default());
            encoder.write_all(&buf1[..])?;

            let payload = encoder.finish()?.into_inner();

            buf.put_u32((payload.len() | COMPRESSION_BIT) as u32);
            buf.unsplit(payload);

            Ok(())
        } else {
            self.encode(buf)?;
            Ok(())
        }
    }
    // 把一个Frame decode成一个Message
    fn decode_frame(buf: &mut BytesMut) -> Result<Self, KvError> {
        let header = buf.get_u32() as usize;
        let (len, compressed) = decode_header(header);
        debug!("Got a frame: msg len {}, compressed {}", len, compressed);

        if compressed {
            // 解压缩
            let mut decoder = GzDecoder::new(&buf[..len]);
            let mut buf1 = Vec::with_capacity(len * 2);
            decoder.read_to_end(&mut buf1)?;
            buf.advance(len);

            // decode 成相应的消息
            Ok(Self::decode(&buf1[..buf1.len()])?)
        } else {
            let msg = Self::decode(&buf[..len])?;
            buf.advance(len);
            Ok(msg)
        }
    }
}

fn decode_header(header: usize) -> (usize, bool) {
    let len = header & !COMPRESSION_BIT;
    let compressed = header & COMPRESSION_BIT == COMPRESSION_BIT;
    (len, compressed)
}

impl FrameCoder for CommandRequest {}

impl FrameCoder for CommandResponse {}

pub async fn read_frame<S>(stream: &mut S, buf: &mut BytesMut) -> Result<(), KvError>
where
    S: AsyncRead + Unpin + Send,
{
    let header = stream.read_u32().await? as usize;
    let (len, compressed) = decode_header(header);
    buf.reserve(LEN_LEN + len);
    buf.put_u32(header as _);

    // advance_mut 是 unsafe 的原因是，从当前位置 pos 到 pos + len，
    // 这段内存目前没有初始化。我们就是为了 reserve 这段内存，然后从 stream
    // 里读取，读取完，它就是初始化的。所以，我们这么用是安全的
    unsafe {
        buf.advance_mut(len);
    }
    stream.read_exact(&mut buf[LEN_LEN..]);

    Ok(())
}
