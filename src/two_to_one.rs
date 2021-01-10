pub fn longest(a1: &str, a2: &str) -> String {
    let full_string = format!("{}{}", a1, a2);
    let mut chars = full_string.chars().collect::<Vec<_>>();

    chars.sort_unstable();
    chars.dedup();

    let chars = chars.iter().collect::<String>();
    chars
}
