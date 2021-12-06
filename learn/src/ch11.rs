use std::collections::HashMap;
use std::io::{BufWriter, Write};
use std::net::TcpStream;

enum E{
    A(f64),
    B(HashMap<String, String>),
    C(Result<Vec<u8>, String>),
}

macro_rules! show_size{
    (header) =>{
        println!(
             "{:<24} {:>4}    {}    {}",
             "Type", "T", "Option<T>", "Result<T, io::Error>"
        )
    }
}

pub(crate) struct MyWriter<W>{
    writer: W,
}

impl <W: Write> MyWriter<W> {
    pub fn new(w: W) -> Self{
        Self{
            writer: w
        }
    }

    pub fn write(&mut self, buf: &str) -> std::io::Result<()>{
        self.writer.write_all(buf.as_bytes())
    }
}