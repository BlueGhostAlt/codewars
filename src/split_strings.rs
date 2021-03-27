pub fn solution(string: &str) -> Vec<String> {
    let chars = string.chars().collect::<Vec<_>>();
    let chunks = chars.chunks(2);

    let chunks = chunks.map(|chunk| {
        let chunk = chunk.iter().collect::<String>();

        format!("{:_<2}", chunk)
    });

    chunks.collect::<Vec<_>>()
}
