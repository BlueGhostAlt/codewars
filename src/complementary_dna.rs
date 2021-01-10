pub fn dna_strand(dna: &str) -> String {
    dna.chars()
        .map(|char| match char {
            'A' => 'T',
            'C' => 'G',
            'G' => 'C',
            'T' => 'A',
            _ => unreachable!(),
        })
        .collect::<String>()
}
