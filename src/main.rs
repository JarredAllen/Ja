#[macro_use]
extern crate clap;

fn main() {
    use clap::App;

    let yml = load_yaml!("arguments.yml");
    let m = App::from_yaml(yml).get_matches();

    if let Some(delay) = m.value_of("DELAY") {
        match delay.parse() {
            Ok(time) => print_delay(m.value_of("STRING").unwrap(), time),
            Err(_) => unreachable!(),
        }
    } else {
        print_no_delay(m.value_of("STRING").unwrap());
    }
}

fn print_delay(text: &str, delay: f64) {
    use std::thread::sleep;
    use std::time::{Duration,Instant};

    let mut time = Instant::now();
    let secs = delay as u64;
    let nanos = ((delay - secs as f64) * 1e9) as u32;
    let duration = Duration::new(secs, nanos);
    loop {
        println!("{}", text);
        time += duration;
        let now = Instant::now();
        if time > now {
            sleep(time.duration_since(now));
        }
    }
}

fn print_no_delay(text: &str) {
    loop {
        println!("{}", text);
    }
}
