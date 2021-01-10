pub fn create_phone_number(numbers: &[u8]) -> String {
    let numbers: String = numbers.iter().map(|i| i.to_string()).collect();

    format!("({}) {}-{}", &numbers[..3], &numbers[3..6], &numbers[6..])
}
