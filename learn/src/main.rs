use std::collections::HashMap;
use std::io::{BufWriter, Write};
use std::net::TcpStream;
use std::str::FromStr;
use std::thread;
use clap::{AppSettings, Parser};
use ch03::fib;
use ch03::return_type::pi;
use anyhow::{anyhow, Result};
use axum::extract::Path;
use axum::handler::get;
use axum::http::StatusCode;
use axum::Router;
use lazy_static::lazy_static;
use percent_encoding::percent_decode_str;
use reqwest::{Client, Url};

use crate::pb::ImageSpec;
use serde::Deserialize;
use ch13::BufBuilder;
use ch14::Developer;
use crate::ch14::{Buffer, Language};
use crate::ch15::{Matrix, MyAllocator};

mod ch03;
mod httpie;
mod thumbor;
mod pb;
mod ch11;
mod ch13;
mod ch14;
mod ch15;
mod ch16;
mod ch19;
mod ch23;
mod ch24;
mod ch33;
mod ch35;
mod file;
mod multi_thread;


use std::{mem::size_of_val};
use std::sync::Mutex;

fn main() {

}

fn new_rectangle(width: u32, height: u32) -> Rectangle {
    Rectangle::new(width, height)
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn new(width: u32, height: u32) -> Self {
        Rectangle {
            width,
            height,
        }
    }
}

fn sum(data: &Vec<u32>) -> u32 {
    println!("addr of value: {:p}, addr of ref: {:p}", data, &data);
    data.iter().fold(0, |acc, x| acc + x)
}

async fn generate(Path(Param { spec, url }): Path<Param>) -> Result<String, StatusCode> {
    let url = percent_decode_str(&url).decode_utf8_lossy();
    let spec: ImageSpec = spec
        .as_str()
        .try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok(format!("url: {}\n ", url, ))
}

#[derive(Deserialize)]
struct Param {
    spec: String,
    url: String,
}


//// A native httpie implementation with rust. can you image how easy it is?
#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "wumin")]
//#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    subCmd: SubCommand,
}

// 子命令对应不同的HTTP方法，支持GET/POST
#[derive(Parser, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
}

//// feed get with an url
#[derive(Parser, Debug)]
struct Get {
    url: String,
}

//// feed post with an url and optional key=value pairs.
#[derive(Parser, Debug)]
struct Post {
    #[clap(parse(try_from_str = parse_url))]
    url: String,
    #[clap(parse(try_from_str = parse_kv_pair))]
    body: Vec<KvPair>,
}

#[derive(Debug)]
struct KvPair {
    k: String,
    v: String,
}

impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("=");
        let err = || anyhow!(format!("Faild to parse {}", s));
        Ok(Self {
            k: (split.next().ok_or_else(err)?).to_string(),
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse()?)
}

fn parse_url(s: &str) -> Result<String> {
    Ok(s.to_string())
}

/*async fn get(client: Client, args: &Get) -> Result<()> {
    let resp = client.get(&args.url).send().await?;
    println!("{:?}", resp.text().await?);
    Ok(())
}*/

async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(&pair.k, &pair.v);
    }

    let resp = client.post(&args.url).json(&body).send().await?;
    println!("{:?}", resp.text().await?);

    Ok(())
}
