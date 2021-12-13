/*
# 指针：一个持有内存地址的值，可以通过解引用来访问他指向的内存地址，可以解引用到任意类型。
# 引用：是一种特殊的指针，解引用受限，只能解引用到他引用的数据类型。
# 智能指针：凡是需要做资源回收的数据结构，且实现了Deref, DerefMut，Drop，都是智能指针

# Cow：用于提供 Clone-on-write 的智能指针。包裹一个只读借用，如果调用者需要所有权或修改内容时，它会Clone数据。
pub enum Cow<'a, B> where: 'a + ToOwned + ?sized{
    Borrowed(&'a B),
    Owned(<B as ToOwned>::Owned),
}






 */

use std::alloc::{GlobalAlloc, Layout, System};
use std::ops::Deref;

pub struct MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let data = System.alloc(layout);
        eprintln!("Allloc: {:p}, size: {}", data, layout.size());
        data
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        eprintln!("Free:{:p}, size:{}", ptr, layout.size());
    }
}

#[allow(dead_code)]
pub struct Matrix {
    /// the length of data is 505, 505 can easy to print by dbg!
    data: [u8; 505],
}

impl Matrix {
    pub fn new() -> Self {
        Self {
            data: [0; 505]
        }
    }
}

impl Default for Matrix {
    fn default() -> Self {
        Self {
            data: [0; 505]
        }
    }
}


const MINI_STRING_MAX_LENGTH: usize = 30;

// MyString 里，String有3个word，共24字节，所以它以8字节对齐
// 所有 enum 的tag + padding最少8字节，整个结构占32字节
// MiniString可以最多30字节（再加上1个字节长度和1字节tag），共32字节
struct MiniString {
    len: u8,
    data: [u8; MINI_STRING_MAX_LENGTH],
}

impl MiniString {
    fn new(v: impl AsRef<str>) -> Self {
        let bytes = v.as_ref().as_bytes();
        // 一定要使用字符串的字节长度
        let len = bytes.len();
        let mut data = [0u8; MINI_STRING_MAX_LENGTH];
        data[..len].copy_from_slice(bytes);
        Self {
            len: len as u8,
            data,
        }
    }
}

use std::{fmt, str};

impl Deref for MiniString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        str::from_utf8(&self.data[..self.len as usize]).unwrap()
    }
}

impl fmt::Debug for MiniString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 这里由于实现了 Deref trait，可以直接得到一个 &str 输出
        write!(f, "{}", self.deref())
    }
}

#[derive(Debug)]
enum MyString {
    Inline(MiniString),
    Standard(String),
}

impl Deref for MyString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match *self {
            MyString::Inline(ref v) => v.deref(),
            MyString::Standard(ref v) => v.deref(),
        }
    }
}

impl From<&str> for MyString {
    fn from(s: &str) -> Self {
        match s.len() > MINI_STRING_MAX_LENGTH {
            true => MyString::Standard(s.to_owned()),
            _ => MyString::Inline(MiniString::new(s)),
        }
    }
}

impl fmt::Display for MyString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}