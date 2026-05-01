use futures::{StreamExt, TryStreamExt};
use reqwest::Client;
use scraper::{Html, Selector};
use tokio::io::{self, AsyncBufReadExt, BufReader, BufWriter};
use url::Url;

const CONCURRENCY: usize = 200;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().user_agent("logo-crawler/0.1").build()?;
    let stdin = BufReader::new(io::stdin());
    let lines = tokio_stream::wrappers::LinesStream::new(stdin.lines());

    let mut writer = io::stdout();

    let mut stream = lines
        .map_ok(|domain| normalize_domain(&domain))
        .try_filter(|d| futures::future::ready(!d.is_empty()))
        .map_ok(|domain| fetch_logo(&client, domain))
        .try_buffer_unordered(CONCURRENCY);

    println!("Hello Async World!");

    Ok(())
}

async fn normalize_domain(input: &str) -> String {
    panic!("not implemented")
}

async fn fetch_logo(client: &Client, domain: String) -> (String, Option<String>) {
    panic!("not implemented")
}
