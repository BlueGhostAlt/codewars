pub mod split_strings;
pub mod dubstep;

fn main() {
    assert_eq!(split_strings::solution("abcdef"), ["ab", "cd", "ef"]);
    assert_eq!(split_strings::solution("abcdefg"), ["ab", "cd", "ef", "g_"]);
    assert_eq!(dubstep::song_decoder("WUBWEWUBAREWUBWUBTHEWUBCHAMPIONSWUBMYWUBFRIENDWUB"), "WE ARE THE CHAMPIONS MY FRIEND")
}
