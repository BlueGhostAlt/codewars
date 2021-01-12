use std::iter::FromIterator;

fn next_permutation<T: std::cmp::PartialOrd + Copy>(array: &[T]) -> Option<Vec<T>> {
    let i = (1..array.len())
        .filter(|&i| array[i - 1] < array[i])
        .max()?;
    let j = (i..array.len())
        .filter(|&j| array[j] > array[i - 1])
        .max()?;

    let mut array = array.to_vec();
    (array[j], array[i - 1]) = (array[i - 1], array[j]);

    let mut prefix = array[..i].to_vec();
    let mut suffix = array[i..].to_vec();
    suffix.reverse();
    prefix.append(&mut suffix);

    let permutation = prefix;

    Some(permutation)
}

pub fn next_bigger_number(n: i64) -> i64 {
    let n = n.to_string();
    let digits = n.chars().collect::<Vec<_>>();

    let next_bigger_permutation = next_permutation(&digits);
    match next_bigger_permutation {
        Some(permutation) => {
            let number = String::from_iter(permutation);
            let number = number.parse::<i64>().unwrap();

            number
        }
        None => -1,
    }
}
