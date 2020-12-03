#![allow(dead_code)]
pub mod feed;
pub mod oms;
pub mod ttcore;
use rust_tensortrade::feed::{BasicStream, Group, Stream};

fn main() {
    let mut stream_a = Stream::source(std::ops::Range { start: 3, end: 5 }.into_iter());
    let mut stream_b = Stream::source(std::ops::Range { start: 1, end: 5 }.into_iter());

    stream_a.rename("STREAM_A".to_string());
    stream_b.rename("STREAM_B".to_string());

    let group = Group::new(Vec::from([stream_a, stream_b]));

    for i in group {
        println!("{:?}", i);
    }

    /*
    let mut clock = ttcore::Clock::new();

    clock.start = 3;

    clock.increment();

    println!("{}", clock.step);
    println!("{}", ttcore::Clock::now());
    println!("{}", ttcore::Clock::format_now("%Y-%m-%d %H:%M:%S"));
    */
}
