use rand::prelude::*;
use std::time::Duration;

const MESSAGES: [&'static str; 5] = [
    "Fish I am a. ",
    "Fish Fish Fish. ",
    "IAMAFISH. ",
    "🐟 🐠 🐡. ",
    "I am a 🐟. ",
];

pub fn get_fishy_sentence() -> &'static str {
    let index = rand::thread_rng().gen_range(0..MESSAGES.len());
    MESSAGES[index]
}

pub fn get_sleep_time() -> Duration {
    let rand_time = rand::thread_rng().gen_range(100..=999);
    Duration::from_millis(rand_time)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_fishy_sentence_returns_valid_message() {
        let message = get_fishy_sentence();
        assert!(MESSAGES.contains(&message));
    }

    #[test]
    fn test_get_fishy_sentence_returns_string_slice() {
        let message = get_fishy_sentence();
        assert!(!message.is_empty());
    }

    #[test]
    fn test_get_sleep_time_returns_valid_duration() {
        let duration = get_sleep_time();
        assert!(duration.as_millis() >= 100);
        assert!(duration.as_millis() <= 999);
    }

    #[test]
    fn test_get_sleep_time_returns_duration_type() {
        let duration = get_sleep_time();
        // Ensure it's a valid Duration (i.e., not zero or infinity)
        assert!(!duration.is_zero());
    }

    #[test]
    fn test_all_messages_are_distinct() {
        let mut seen = std::collections::HashSet::new();
        for msg in MESSAGES {
            assert!(seen.insert(msg), "Duplicate message found: {}", msg);
        }
    }
}
