pub fn range_extraction(a: &[i32]) -> String {
    let mut groups: Vec<Vec<i32>> = Vec::new();

    for &num in a {
        let current_group = groups.last_mut();
        match current_group {
            Some(group) => {
                let last = group.last().unwrap();
                if num - last == 1 {
                    group.push(num);
                } else {
                    let mut group = Vec::new();
                    group.push(num);
                    groups.push(group);
                }
            },
            None => {
                let mut group = Vec::new();
                group.push(num);
                groups.push(group);
            }
        }
    }

    let strings = groups
        .iter()
        .map(|group| match group.len() {
            1 => group.first().unwrap().to_string(),
            2 => format!("{},{}", group[0], group[1]),
            _ => format!("{}-{}", group.first().unwrap(), group.last().unwrap()),
        })
        .collect::<Vec<_>>();

    let string = strings.join(",");

    string
}
