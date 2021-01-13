pub fn wave(s: &str) -> Vec<String> {
    s.char_indices()
        .map(|(i, c)| {
            format!(
                "{}{}{}",
                &s[..i],
                c.to_uppercase().collect::<String>(),
                &s[i + 1..]
            )
        })
        .filter(|wave| wave != s)
        .collect::<Vec<_>>()
}
