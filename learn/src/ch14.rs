/****     trait       ***/
/*
### 内存相关：Clone Copy Drop

# Clone
pub trait Clone {
    fn clone(&self) -> Self;
    fn clone_from(&mut self, source: &Self) {
        *self = source.clone();
    }
}
clone_from有缺省实现。对一个已存在的对象clone，使用clone_from可以避免内存分配，提高效率
Clone 是深度拷贝，栈内存和堆内存一起拷贝。

# Copy
pub trait Copy: Clone {}

要实现Copy必须实现Clone trait。Copy没有任何行为，是标记trait

# Drop
pub trait Drop {
    fn drop(&mut self);
}
使用场景：
1、希望在结束生命周期的时候做一些事，如记日志。
2、需要资源回收的场景，如锁的释放。

Copy和Drop是互斥的。

### 标记trait：Sized Send Sync Unpin

# Sized
Sized 用于标记有具体大小的类型。使用泛型参数时，Rust编译器会自动为泛型参数加上Sized的约束。
在少数情况下，需要T是可变类型的，Rust提供 ?Sized来摆脱约束。

# Send
pub unsafe auto trait Send {}
# Sync
pub unsafe auto trait Sync {}

auto 标识编译器会在合适的场合自动为数据结构添加他们的实现。
unsafe 标识这个trait可能违背了Rust的内存安全准则。

1、如果类型T实现了Send trait，表示T可以安全的从一个线程移动到另一个线程。
    如果一个类型T：Send，那么T在某个线程的独占访问是线程安全的。
2、如果一个类型实现了Sync trait，表示&T可以安全的在多个线程中共享
    如果一个类型T：Sync，那么T在线程间只读共享是安全的。

自定义的数据结构，如果内部的所有域都实现了Send / Sync，那么这个数据机构也会自动添加Send / Sync。
基本上原生数据结构都支持Send / Sync。
不支持Send / Sync的数据结构有：
    裸指针*const T / *mut T。他们是不安全的，所以不是Send / Sync
    UnsafeCell<T> 不支持Sync。Cell， RefCell。
    引用计数Rc不支持Send / Sync。所以Rc无法跨线程。

### 类型转换：From<T>  Into<T> AsRef<T>  AsMut<T>
pub trait From<T>{
    fn from(T) -> Self;
}
pub trait Into<T> {
    fn into(self) -> T;
}
实现From<T>会自动实现Into<T>，所以不需要实现Into<T>，只要实现From<T>。

AsRef<T> AsMut<T> 用于从引用到引用的转换。

### 操作符相关：Deref, DerefMut







 */



use std::ops::{Deref, DerefMut};


#[derive(Clone, Debug)]
pub(crate) struct Developer{
    pub name: String,
    pub age: u8,
    pub lang: Language
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Language{
    Rust,
    TypeScript,
    JAVA,
    Haskell,
}

#[derive(Debug)]
pub struct Buffer<T>(Vec<T>);

impl<T> Buffer<T> {
    pub fn new(v: impl Into<Vec<T>>) -> Buffer<T> {
        Self(v.into())
    }
}

impl<T> Deref for Buffer<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Buffer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
