use std::collections::HashMap;
use std::str::FromStr;
use clap::{AppSettings, Parser};
use ch03::fib;
use ch03::return_type::pi;
use anyhow::{anyhow, Result};
use axum::extract::Path;
use axum::handler::get;
use axum::http::StatusCode;
use axum::Router;
use percent_encoding::percent_decode_str;
use reqwest::Client;
use crate::pb::ImageSpec;
use serde::Deserialize;

mod ch03;
mod httpie;
mod thumbor;
mod pb;


#[tokio::main]
async fn main() {
    // scrape::scrape("https://www.rust-lang.org/")
    /*
    let pi = return_type::pi();
    let not_pi = return_type::not_pi();

    let pi2 = {
        return_type::pi();
    };
    println!("pi:{:?}, not_pi:{:?}, pi2:{:?}", pi, not_pi, pi2);
     */

    // echo::test();
    /*
        let result = fib::fib_loop(6);
        println!("result: {}", result);*/

    //  httpie  --start
    /*let opt: Opts = Opts::parse();
    println!("{:?}", opt);
    let client = Client::new();
    let result = match opt.subCmd {
        SubCommand::Get(ref args) => get(client, args).await?,
        SubCommand::Post(ref args) => post(client, args).await?,
    };
    Ok(result)*/
    //  httpie  --end
    //  thumbor  --start
    /*tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/image/:spec/:url", get(generate));
    let addr = "127.0.0.1:3000".parse().unwrap();
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
*/
    //  thumbor  --end
  /*  let data: Vec<u32> = vec![1, 2, 3, 4];
    let data1 = &data;
    println!("addr of value: {:p}({:p}), addr of data: {:p}, data1: {:p}", &data, data1, &&data, &data1);
    sum(data1);
    // 堆上数据的地址是什么？
    println!("addr of items: [{:p}, {:p}, {:p}, {:p}]", &data[0], &data[1], &data[2], &data[3]);

   */
    let rect1 = Rectangle{
        width: 10,
        height: 12,
    };

    print!("rect1 is {:#?}", rect1);

}

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
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
    Ok(format!("url: {}\n spec: {:#?}", url, spec))
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
