mod ttcore;

fn main() {
    let mut clock = ttcore::Clock::new();

    clock.start = 3;

    clock.increment();

    println!("{}", clock.step);
    println!("{}", ttcore::Clock::now());
    println!("{}", ttcore::Clock::format_now("%Y-%m-%d %H:%M:%S"));
}
