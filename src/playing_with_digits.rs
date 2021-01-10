pub fn dig_pow(n: i64, p: i32) -> i64 {
    let n_string = n.to_string();
    let digits = n_string.chars().enumerate();
    let digits = digits
        .map(|(i, d)| ((i as i32 + p) as u32, d))
        .map(|(p, d)| {
            let d = d.to_digit(10).unwrap() as i64;
            d.pow(p)
        });
    let sum = digits.sum::<i64>();

    let k = sum / n;
    if k == 0 || sum / k != n {
        return -1
    }

    k
}
