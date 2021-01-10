pub mod split_strings;
pub mod dubstep;
pub mod regex_validate_pin_code;
pub mod two_to_one;
pub mod descending_order;
pub mod which_are_in;
pub mod complementary_dna;
pub mod playing_with_digits;

fn main() {
    assert_eq!(split_strings::solution("abcdef"), ["ab", "cd", "ef"]);
    assert_eq!(split_strings::solution("abcdefg"), ["ab", "cd", "ef", "g_"]);
    assert_eq!(dubstep::song_decoder("WUBWEWUBAREWUBWUBTHEWUBCHAMPIONSWUBMYWUBFRIENDWUB"), "WE ARE THE CHAMPIONS MY FRIEND");
    assert!(regex_validate_pin_code::validate_pin("1234"));
    assert!(!regex_validate_pin_code::validate_pin("12345"));
    assert!(!regex_validate_pin_code::validate_pin("a2345"));
    assert_eq!(two_to_one::longest("xyaabbbccccdefww", "xxxxyyyyabklmopq"), "abcdefklmopqwxy");
    assert_eq!(two_to_one::longest("abcdefghijklmnopqrstuvwxyz", "abcdefghijklmnopqrstuvwxyz"), "abcdefghijklmnopqrstuvwxyz");
    assert_eq!(descending_order::descending_order(42145), 54421);
    assert_eq!(descending_order::descending_order(145263), 654321);
    assert_eq!(descending_order::descending_order(123456789), 987654321);
    assert_eq!(which_are_in::in_array(&["arp", "live", "strong"], &["lively", "alive", "harp", "sharp", "armstrong"]), ["arp", "live", "strong"]);
    assert_eq!(which_are_in::in_array(&["tarp", "mice", "bull"], &["lively", "alive", "harp", "sharp", "armstrong"]), [] as [&str; 0]);
    assert_eq!(complementary_dna::dna_strand("ATTGC"), "TAACG");
    assert_eq!(complementary_dna::dna_strand("GTAT"), "CATA");
    assert_eq!(playing_with_digits::dig_pow(89, 1), 1);
    assert_eq!(playing_with_digits::dig_pow(92, 1), -1);
    assert_eq!(playing_with_digits::dig_pow(695, 2), 2);
    assert_eq!(playing_with_digits::dig_pow(46288, 3), 51)
}
