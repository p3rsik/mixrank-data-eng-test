use futures::{StreamExt, TryStreamExt};
use reqwest::Client;
use scraper::{Html, Selector};
use tokio::io::{self, AsyncBufReadExt, BufReader};
use url::Url;

const CONCURRENCY: usize = 200;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().user_agent("logo-crawler/0.1").build()?;
    let stdin = BufReader::new(io::stdin());
    let lines = tokio_stream::wrappers::LinesStream::new(stdin.lines());

    // do I need a handle to stdout, or can I just use println?
    // let mut writer = io::stdout();

    let mut stream = lines
        .map_ok(|domain| normalize_domain(&domain))
        .try_filter(|d| futures::future::ready(!d.is_empty()))
        .map_ok(|domain| fetch_logo(&client, domain))
        .try_buffer_unordered(CONCURRENCY);

    while let Some(result) = stream.next().await {
        if let Ok((domain, logo)) = result {
            if let Some(l) = logo {
                println!("{domain};{l}");
            } else {
                println!("{domain};")
            }
        }
    }

    println!("Hello Async World!");

    Ok(())
}

fn normalize_domain(input: &str) -> String {
    panic!("not implemented")
}

async fn fetch_logo(
    client: &Client,
    domain: String,
) -> Result<(String, Option<String>), std::io::Error> {
    panic!("not implemented")
}
