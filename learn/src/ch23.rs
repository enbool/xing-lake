use std::io::Read;
use std::marker::PhantomData;
use std::sync::Arc;

/// 逐步对泛型数据结构约束
pub struct BufReader<R>{
    inner: R,
    buf: Box<[u8]>,
    pos: usize,
    cap: usize,
}

// capacity 和buffer不需要使用 R 的任何特殊能力
impl<R> BufReader<R> {
    pub fn capacity(&self) -> usize {
        self.cap
    }
    pub fn buffer(&self) -> &[u8] {
        &self.buf
    }
}

// new 的时候使用了Read trait
/*impl<R: Read> BufReader<R> {
    pub fn new(mut inner: R) -> Self {
        Self{
            inner,
            buf: Box::new(inner.read_u8().unwrap()?),
            pos: 0,
            cap: 0,
        }
    }
}*/

/// 泛型参数的三种使用场景 <br>
/// 1、使用泛型参数延迟数据结构的绑定 <br>
/// 2、使用泛型参数和PhantomData，声明参数在数据结构中不直接使用，但在实现过程中需要使用到的类型 <br>
/// 3、使用泛型参数让同一个数据结构对同一个trait可以拥有不同的实现

// 1、延迟绑定
pub struct Service<Store = MemTable>{
    inner: Arc<ServiceInner<Store>>,
}

struct ServiceInner<T>{
    _tag: PhantomData<T>,
}

impl<T> ServiceInner<T> {
    fn new() -> Self{
        Self{ _tag: Default::default() }
    }
}

pub struct MemTable{}

impl<Store> Service<Store> {
    pub fn new() -> Self{
        Self{
            inner: Arc::new(ServiceInner::new()),
        }
    }
}

impl<Store: Storage> Service<Store> {
    pub fn execute(){}
}

pub trait Storage{}

// 2、PhantomData
pub struct Identifier<T>{
    inner: u64,
    _tag: PhantomData<T>
}

// 3、使用泛型参数提供多个实现
// 参考AsyncProstReader
// 除了 Stream 的实现不同外，
// AsyncProstReader 的其它实现都是共享的。
// 所以我们有必要为其增加一个泛型参数 D，
// 使其可以根据不同的 D 的类型，来提供不同的 Stream 实现。


/// 返回值带泛型参数 <br>
/// Rust 目前不支持在返回值中使用 impl trait
///
struct Nothing;
