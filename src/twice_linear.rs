pub fn dbl_linear(n: u32) -> u32 {
    let n = n as usize;
    let mut vec = Vec::with_capacity(n);
    vec.push(1);

    let mut x = 0;
    let mut y = 0;

    for _ in 0..n {
        let a = 2 * vec[x] + 1;
        let b = 3 * vec[y] + 1;

        if a > b {
            vec.push(b);
            y += 1;
        } else if a < b {
            vec.push(a);
            x += 1;
        } else {
            vec.push(a);
            x += 1;
            y += 1;
        }
    }

    vec[n]
}
