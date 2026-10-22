use std::io::{self, Write};
use std::thread::sleep;

use i_am_a_fish::{get_fishy_sentence, get_sleep_time};

fn main() -> io::Result<()> {
    for _ in 0..400 {
        for letter in get_fishy_sentence().chars() {
            write!(io::stdout(), "{}", letter)?;
            io::stdout().flush()?;
        }
        sleep(get_sleep_time());
    }
    Ok(())
}
