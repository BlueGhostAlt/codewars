use std::cmp::Ordering;

pub fn dbl_linear(n: u32) -> u32 {
    let n = n as usize;
    let mut vec = Vec::with_capacity(n);
    vec.push(1);

    let mut i1 = 0;
    let mut i2 = 0;

    for _ in 0..n {
        let a = 2 * vec[i1] + 1;
        let b = 3 * vec[i2] + 1;

        match a.cmp(&b) {
            Ordering::Greater => {
                vec.push(b);
                i2 += 1;
            }
            Ordering::Less => {
                vec.push(a);
                i1 += 1;
            }
            Ordering::Equal => {
                vec.push(a);
                i1 += 1;
                i2 += 1;
            }
        }
    }

    vec[n]
}
