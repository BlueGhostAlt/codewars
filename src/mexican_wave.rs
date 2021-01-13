fn capitalize_letter(word: &str, index: usize) -> String {
    word.chars()
        .enumerate()
        .map(|(i, c)| {
            if i == index {
                c.to_uppercase().collect::<String>()
            } else {
                c.to_string()
            }
        })
        .collect::<String>()
}

pub fn wave(s: &str) -> Vec<String> {
    let len = s.chars().count();
    let waves = (0..len)
        .map(|i| capitalize_letter(s, i))
        .filter(|wave| wave != s)
        .collect::<Vec<_>>();

    waves
}
