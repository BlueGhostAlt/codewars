pub fn order(sentence: &str) -> String {
    let mut words = sentence
        .split_whitespace()
        .map(String::from)
        .collect::<Vec<_>>();

    words.sort_unstable_by_key(|word| word.chars().find(|c| c.is_digit(10)).unwrap_or('\0'));

    words.join(" ")
}
