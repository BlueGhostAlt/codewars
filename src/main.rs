pub mod split_strings;

fn main() {
    assert_eq!(split_strings::solution("abcdef"), ["ab", "cd", "ef"]);
    assert_eq!(split_strings::solution("abcdefg"), ["ab", "cd", "ef", "g_"]);
}
