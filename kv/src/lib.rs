/*                    ————————————————————————————————————————————————————————————————————————————————————————
                      |                                                   --------------                     |
         request      |   TCP               序列化，反序列化                |  ---> hset  |                     |
client <----------->  |   IO  <------------->  SerDe  ------> Dispatcher |  ---> hget  |  <-----> Storage    |
         response     |   HTTP                                           |   ...       |                     |
                      |   gRPC                                           |-------------|                     |
                      |                                                      事  |  件                        |
                      |                                                         V                            |
                      |   Server                                      logger | WAL | stats                   |
                      ————————————————————————————————————————————————————————————————————————————————————————

1、客户端与服务端用什么协议通信？TCP、gRPC、HTTP
2、客户端和服务端之间交互的应用层协议应该如何定义？怎么序列化/反序列化？Protobuf、JSON
3、服务器都支持哪些命令
4、具体处理逻辑中，需不需要加hook
5、对于存储，要支持不同的存储引擎吗？如MemDB、RocksDB、对于内存存储，我们考虑WAL
6、整个系统可配置吗？如服务端口，存储引擎

 */

mod channel;
mod client;
mod error;
mod network;
mod pb;
mod server;
mod service;
mod storage;
mod test;

pub use error::KvError;
pub use network::*;
pub use pb::abi::*;
pub use service::*;
pub use storage::*;
