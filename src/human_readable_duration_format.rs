const SECOND: u64 = 1;
const MINUTE: u64 = 60 * SECOND;
const HOUR: u64 = 60 * MINUTE;
const DAY: u64 = 24 * HOUR;
const YEAR: u64 = 365 * DAY;

pub fn format_duration(seconds: u64) -> String {
    let (years, seconds) = (seconds / YEAR, seconds % YEAR);
    let (days, seconds) = (seconds / DAY, seconds % DAY);
    let (hours, seconds) = (seconds / HOUR, seconds % HOUR);
    let (minutes, seconds) = (seconds / MINUTE, seconds % MINUTE);

    let duration = vec![years, days, hours, minutes, seconds];
    let mut duration = duration
        .into_iter()
        .enumerate()
        .filter(|&(_, u)| u != 0)
        .map(|(i, u)| {
            let unit = match i {
                0 => "year",
                1 => "day",
                2 => "hour",
                3 => "minute",
                4 => "second",
                _ => unreachable!(),
            };

            if u > 1 {
                format!("{} {}s", u, unit)
            } else {
                format!("1 {}", unit)
            }
        })
        .collect::<Vec<_>>();

    match duration.len() {
        0 => String::from("now"),
        _ => {
            let last = duration.pop().unwrap();

            let duration = duration.join(", ");

            if duration.is_empty() {
                last
            } else {
                format!("{} and {}", duration, last)
            }
        }
    }
}
