pub fn in_array(arr_a: &[&str], arr_b: &[&str]) -> Vec<String> {
    let mut included_strings = arr_a
        .iter()
        .filter(|&a| arr_b.iter().any(|b| b.contains(a)))
        .map(|&a| String::from(a))
        .collect::<Vec<_>>();

    included_strings.sort_unstable();
    included_strings.dedup();

    included_strings
}
