pub mod split_strings;
pub mod dubstep;
pub mod regex_validate_pin_code;
pub mod two_to_one;

fn main() {
    assert_eq!(split_strings::solution("abcdef"), ["ab", "cd", "ef"]);
    assert_eq!(split_strings::solution("abcdefg"), ["ab", "cd", "ef", "g_"]);
    assert_eq!(dubstep::song_decoder("WUBWEWUBAREWUBWUBTHEWUBCHAMPIONSWUBMYWUBFRIENDWUB"), "WE ARE THE CHAMPIONS MY FRIEND");
    assert!(regex_validate_pin_code::validate_pin("1234"));
    assert!(!regex_validate_pin_code::validate_pin("12345"));
    assert!(!regex_validate_pin_code::validate_pin("a2345"));
    assert_eq!(two_to_one::longest("xyaabbbccccdefww", "xxxxyyyyabklmopq"), "abcdefklmopqwxy");
    assert_eq!(two_to_one::longest("abcdefghijklmnopqrstuvwxyz", "abcdefghijklmnopqrstuvwxyz"), "abcdefghijklmnopqrstuvwxyz")
}
