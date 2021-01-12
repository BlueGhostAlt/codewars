use std::collections::HashMap;

pub fn zeroes(base: i32, number: i32) -> i32 {
    let mut prime_factors = HashMap::new();
    let mut base = base;
    for divisor in 2..=base {
        while base % divisor == 0 {
            if let Some(count) = prime_factors.get_mut(&divisor) {
                *count += 1;
            } else {
                prime_factors.insert(divisor, 1);
            }
            base /= divisor;
        }
    }

    let mut counts = HashMap::new();
    prime_factors.keys().for_each(|&key| {
        counts.insert(
            key,
            (1..((number as f64).log(key as f64) + 1.0) as i32)
                .map(|i| number / key.pow(i as u32))
                .sum::<i32>(),
        );
    });


    let zeros_count = prime_factors
        .keys()
        .map(|k| counts[k] / prime_factors[k])
        .min()
        .unwrap();

    zeros_count
}
