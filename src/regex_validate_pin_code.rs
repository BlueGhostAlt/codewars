pub fn validate_pin(pin: &str) -> bool {
    let is_only_digits = pin.chars().all(|char| char.is_digit(10));
    let length = pin.len();
    is_only_digits && (length == 4 || length == 6)
}
