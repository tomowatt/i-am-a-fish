use rand::prelude::*;
use std::io::stdout;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

fn get_fishy_sentence() -> &'static str {
    let rnd_num = rand::thread_rng().gen_range(0..=100);
    match rnd_num {
        0..=10 => "Fish I am a. ",
        11..=16 => "Fish Fish Fish. ",
        21..=25 => "IAMAFISH. ",
        33 => "🐟 🐠 🐡. ",
        95..=100 => "I am a 🐟. ",
        _ => "I am a fish. ",
    }
}

fn get_sleep_time() -> Duration {
    let rand_time = rand::thread_rng().gen_range(100..1000);
    Duration::from_millis(rand_time)
}

fn main() {
    for _ in 0..400 {
        for letter in get_fishy_sentence().chars() {
            print!("{}", letter);
            stdout().flush().unwrap();
        }
        sleep(get_sleep_time());
    }
}
