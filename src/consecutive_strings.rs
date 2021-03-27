pub fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
    if k == 0 {
        return String::new();
    }

    let windows = strarr.windows(k);
    let windows = windows.map(|window| window.concat()).rev();

    windows
        .max_by_key(|window| window.len())
        .unwrap_or_default()
}
