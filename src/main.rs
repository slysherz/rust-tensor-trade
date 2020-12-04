#![allow(dead_code)]

use rust_tensortrade::{data::crypto_data_download, feed::Stream, oms::exchanges::StreamLike};
pub mod data;
pub mod feed;
pub mod oms;
pub mod ttcore;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = crypto_data_download::fetch_default(
        "Coinbase".to_string(),
        "USD".to_string(),
        "BTC".to_string(),
        "1h".to_string(),
    )?;

    let data_stream = Stream::source(data.iter());
    let prices: Box<dyn StreamLike<Item = f32>> = Box::new(Stream::apply(data_stream, |d| d.close));

    let _streams: Vec<Box<dyn StreamLike<Item = f32>>> = Vec::from([prices]);

    let coinbase = Exchange::new(
        "coinbase".to_string(),
        None,
        ExchangeOptions::new(),
        streams,
    );
    /*

    let mut clock = ttcore::Clock::new();

    clock.start = 3;

    clock.increment();

    println!("{}", clock.step);
    println!("{}", ttcore::Clock::now());
    println!("{}", ttcore::Clock::format_now("%Y-%m-%d %H:%M:%S"));
    */

    Ok(())
}
