#![allow(dead_code)]
use rust_tensortrade::feed::{BasicStream, Group, Stream};

#[test]
fn test_streams() {
    let mut stream_a = Stream::source(std::ops::Range { start: 3, end: 5 }.into_iter());
    let mut stream_b = Stream::source(std::ops::Range { start: 1, end: 5 }.into_iter());

    stream_a.rename("STREAM_A".to_string());
    stream_b.rename("STREAM_B".to_string());

    let group = Group::new(Vec::from([stream_a, stream_b]));

    for i in group {
        println!("{:?}", i);
    }
}
