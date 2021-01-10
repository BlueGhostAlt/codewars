pub mod complementary_dna;
pub mod consecutive_strings;
pub mod create_phone_number;
pub mod descending_order;
pub mod dubstep;
pub mod is_my_friend_cheating;
pub mod playing_with_digits;
pub mod regex_validate_pin_code;
pub mod split_strings;
pub mod two_to_one;
pub mod which_are_in;

fn main() {
    assert_eq!(complementary_dna::dna_strand("ATTGC"), "TAACG");
    assert_eq!(complementary_dna::dna_strand("GTAT"), "CATA");

    assert_eq!(consecutive_strings::longest_consec(vec!["tree", "foling", "trashy", "blue", "abcdef", "uvwxyz"], 2), "folingtrashy");
    assert_eq!(consecutive_strings::longest_consec(vec!["zone", "abigail", "theta", "form", "libe", "zas", "theta", "abigail"], 2), "abigailtheta");

    assert_eq!(create_phone_number::create_phone_number(&[1,2,3,4,5,6,7,8,9,0]), "(123) 456-7890");

    assert_eq!(descending_order::descending_order(42145), 54421);
    assert_eq!(descending_order::descending_order(145263), 654321);
    assert_eq!(descending_order::descending_order(123456789), 987654321);

    assert_eq!(dubstep::song_decoder("WUBWEWUBAREWUBWUBTHEWUBCHAMPIONSWUBMYWUBFRIENDWUB"), "WE ARE THE CHAMPIONS MY FRIEND");

    assert_eq!(is_my_friend_cheating::remove_nb(26), vec![(15, 21), (21, 15)]);

    assert_eq!(playing_with_digits::dig_pow(89, 1), 1);
    assert_eq!(playing_with_digits::dig_pow(92, 1), -1);
    assert_eq!(playing_with_digits::dig_pow(695, 2), 2);
    assert_eq!(playing_with_digits::dig_pow(46288, 3), 51);

    assert!(regex_validate_pin_code::validate_pin("1234"));
    assert!(!regex_validate_pin_code::validate_pin("12345"));
    assert!(!regex_validate_pin_code::validate_pin("a2345"));

    assert_eq!(split_strings::solution("abcdef"), ["ab", "cd", "ef"]);
    assert_eq!(split_strings::solution("abcdefg"), ["ab", "cd", "ef", "g_"]);

    assert_eq!(two_to_one::longest("xyaabbbccccdefww", "xxxxyyyyabklmopq"), "abcdefklmopqwxy");
    assert_eq!(two_to_one::longest("abcdefghijklmnopqrstuvwxyz", "abcdefghijklmnopqrstuvwxyz"), "abcdefghijklmnopqrstuvwxyz");

    assert_eq!(which_are_in::in_array(&["arp", "live", "strong"], &["lively", "alive", "harp", "sharp", "armstrong"]), ["arp", "live", "strong"]);
    assert_eq!(which_are_in::in_array(&["tarp", "mice", "bull"], &["lively", "alive", "harp", "sharp", "armstrong"]), [] as [&str; 0]);
}
