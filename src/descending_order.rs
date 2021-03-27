pub fn descending_order(x: u64) -> u64 {
    let mut digits = x.to_string().chars().collect::<Vec<_>>();
    digits.sort_unstable();
    digits.reverse();

    let new_number = digits.iter().collect::<String>();

    new_number.parse::<u64>().unwrap_or(x)
}
