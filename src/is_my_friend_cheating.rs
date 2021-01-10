pub fn remove_nb(m: i32) -> Vec<(i32, i32)> {
    let n = m as u64;
    let sum = n * (n + 1) / 2;

    let lower_bound = ((n - 1) * n / 2) / (n + 1);
    let upper_bound = ((sum + 1) as f64).sqrt().trunc() as u64 - 1;

    let mut tuples = Vec::new();
    for a in (lower_bound..=upper_bound).rev() {
        let b = (sum - a) / (a + 1);
        if a * b + a + b == sum {
            tuples.insert(0, (a as i32, b as i32));
            tuples.push((b as i32, a as i32));
        }
    }

    tuples
}
