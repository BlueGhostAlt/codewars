#![feature(box_patterns)]
#![feature(box_syntax)]
#![feature(destructuring_assignment)]

pub mod algebraic_lists;
pub mod complementary_dna;
pub mod consecutive_strings;
pub mod create_phone_number;
pub mod descending_order;
pub mod dubstep;
pub mod factorial_tail;
pub mod is_my_friend_cheating;
pub mod mexican_wave;
pub mod next_bigger_number_with_the_same_digits;
pub mod playing_with_digits;
pub mod range_extraction;
pub mod regex_validate_pin_code;
pub mod split_strings;
pub mod two_to_one;
pub mod which_are_in;
pub mod your_order_please;

fn main() {
    {
        let numbers = algebraic_lists::Cons::from_iter(vec![1, 2, 3, 4, 5]);
        assert_eq!(numbers.filter(|x| x % 2 == 0).to_vec(), vec![2, 4]);
        assert_eq!(numbers.map(|x| x * x).to_vec(), vec![1, 4, 9, 16, 25]);

        let digits = algebraic_lists::Cons::from_iter(vec!["1", "2", "3", "4", "5"]);
        let ints = digits
            .map(str::parse::<i32>)
            .map(Result::unwrap)
            .filter(|&n| n > 3);
        assert_eq!(ints.to_vec(), vec![4, 5]);
    }

    assert_eq!(complementary_dna::dna_strand("ATTGC"), "TAACG");
    assert_eq!(complementary_dna::dna_strand("GTAT"), "CATA");

    assert_eq!(consecutive_strings::longest_consec(vec!["tree", "foling", "trashy", "blue", "abcdef", "uvwxyz"], 2), "folingtrashy");
    assert_eq!(consecutive_strings::longest_consec(vec!["zone", "abigail", "theta", "form", "libe", "zas", "theta", "abigail"], 2), "abigailtheta");

    assert_eq!(create_phone_number::create_phone_number(&[1,2,3,4,5,6,7,8,9,0]), "(123) 456-7890");

    assert_eq!(descending_order::descending_order(42145), 54421);
    assert_eq!(descending_order::descending_order(145263), 654321);
    assert_eq!(descending_order::descending_order(123456789), 987654321);

    assert_eq!(dubstep::song_decoder("WUBWEWUBAREWUBWUBTHEWUBCHAMPIONSWUBMYWUBFRIENDWUB"), "WE ARE THE CHAMPIONS MY FRIEND");

    assert_eq!(factorial_tail::zeroes(10, 10), 2);
    assert_eq!(factorial_tail::zeroes(16, 16), 3);

    assert_eq!(is_my_friend_cheating::remove_nb(26), vec![(15, 21), (21, 15)]);

    assert_eq!(mexican_wave::wave("hello"), vec!["Hello", "hEllo", "heLlo", "helLo", "hellO"]);

    assert_eq!(next_bigger_number_with_the_same_digits::next_bigger_number(12), 21);
    assert_eq!(next_bigger_number_with_the_same_digits::next_bigger_number(513), 531);
    assert_eq!(next_bigger_number_with_the_same_digits::next_bigger_number(2017), 2071);

    assert_eq!(playing_with_digits::dig_pow(89, 1), 1);
    assert_eq!(playing_with_digits::dig_pow(92, 1), -1);
    assert_eq!(playing_with_digits::dig_pow(695, 2), 2);
    assert_eq!(playing_with_digits::dig_pow(46288, 3), 51);

    assert_eq!(range_extraction::range_extraction(&[-6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20]), "-6,-3-1,3-5,7-11,14,15,17-20");
    assert_eq!(range_extraction::range_extraction(&[-3, -2, -1, 2, 10, 15, 16, 18, 19, 20]), "-3--1,2,10,15,16,18-20");

    assert!(regex_validate_pin_code::validate_pin("1234"));
    assert!(!regex_validate_pin_code::validate_pin("12345"));
    assert!(!regex_validate_pin_code::validate_pin("a2345"));

    assert_eq!(split_strings::solution("abcdef"), ["ab", "cd", "ef"]);
    assert_eq!(split_strings::solution("abcdefg"), ["ab", "cd", "ef", "g_"]);

    assert_eq!(two_to_one::longest("xyaabbbccccdefww", "xxxxyyyyabklmopq"), "abcdefklmopqwxy");
    assert_eq!(two_to_one::longest("abcdefghijklmnopqrstuvwxyz", "abcdefghijklmnopqrstuvwxyz"), "abcdefghijklmnopqrstuvwxyz");

    assert_eq!(which_are_in::in_array(&["arp", "live", "strong"], &["lively", "alive", "harp", "sharp", "armstrong"]), ["arp", "live", "strong"]);
    assert_eq!(which_are_in::in_array(&["tarp", "mice", "bull"], &["lively", "alive", "harp", "sharp", "armstrong"]), [] as [&str; 0]);

    assert_eq!(your_order_please::order("is2 Thi1s T4est 3a"), "Thi1s is2 3a T4est");
    assert_eq!(your_order_please::order("4of Fo1r pe6ople g3ood th5e the2"), "Fo1r the2 g3ood 4of th5e pe6ople");
    assert_eq!(your_order_please::order(""), "");
}
