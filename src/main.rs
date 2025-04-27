use std::io::stdout;
use std::io::Write;
use std::thread::sleep;

use i_am_a_fish::{get_fishy_sentence, get_sleep_time};

fn main() {
    for _ in 0..400 {
        for letter in get_fishy_sentence().chars() {
            print!("{}", letter);
            stdout().flush().unwrap();
        }
        sleep(get_sleep_time());
    }
}
