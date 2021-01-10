pub fn song_decoder(song: &str) -> String {
    let original = song.replace("WUB", " ");
    let original = original.trim();

    let original_words = original.split(" ");
    let original_non_empty_words = original_words.filter(|word| word != &"");
    let original_words = original_non_empty_words.collect::<Vec<_>>();

    let original = original_words.join(" ");

    original
}
