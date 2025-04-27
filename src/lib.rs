use rand::prelude::*;
use std::time::Duration;

pub fn get_fishy_sentence() -> &'static str {
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

pub fn get_sleep_time() -> Duration {
    let rand_time = rand::thread_rng().gen_range(100..1000);
    Duration::from_millis(rand_time)
}