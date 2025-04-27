use i_am_a_fish::{get_fishy_sentence, get_sleep_time};

#[test]
fn test_get_fishy_sentence() {
    let sentence = get_fishy_sentence();
    let possible_sentences = vec![
        "Fish I am a. ",
        "Fish Fish Fish. ",
        "IAMAFISH. ",
        "🐟 🐠 🐡. ",
        "I am a 🐟. ",
        "I am a fish. ",
    ];
    assert!(possible_sentences.contains(&sentence));
}

#[test]
fn test_get_sleep_time() {
    let duration = get_sleep_time();
    assert!(duration.as_millis() >= 100 && duration.as_millis() <= 1000);
}