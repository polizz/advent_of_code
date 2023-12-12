pub fn get_winning_wait_times(races: impl Iterator<Item = RaceStat>) -> Vec<Vec<usize>> {
    races
        // .inspect(|x| println!("x: {:?}", x))
        .filter_map(
            |RaceStat {
                 duration,
                 max_distance,
             }| {
                let wins = (1..duration)
                    .filter_map(|wait| {
                        let speed = wait;
                        let total_distance = speed * (duration - wait);
                        if total_distance > max_distance {
                            return Some(wait);
                        }

                        None
                    })
                    .collect::<Vec<usize>>();

                if wins.len() > 0 {
                    return Some(wins);
                }

                None
            },
        )
        .collect::<Vec<Vec<usize>>>()
}

pub fn get_race_stats2<'a>(input: &'a str) -> impl Iterator<Item = RaceStat> + 'a {
    let mut lines = input.lines();
    let times = lines.next().unwrap().split_whitespace();
    let durations = lines.next().unwrap().split_whitespace();

    let time = times
        .zip(durations)
        .skip(1)
        // .inspect(|x| println!("x is: {:?}", x))
        .fold(("".to_owned(), "".to_owned()), |mut acc, pair| {
            acc.0 = format!("{}{}", acc.0, pair.0);
            acc.1 = format!("{}{}", acc.1, pair.1);

            acc
        });

    vec![RaceStat {
        duration: time.0.parse().expect("Invalid number for duration"),
        max_distance: time.1.parse().expect("Invalid number for distance"),
    }]
    .into_iter()
}

pub fn get_race_stats<'a>(input: &'a str) -> impl Iterator<Item = RaceStat> + 'a {
    let mut lines = input.lines();
    let times = lines.next().unwrap().split_whitespace();
    let durations = lines.next().unwrap().split_whitespace();

    times.zip(durations).skip(1).map(|pair| RaceStat {
        duration: pair.0.parse::<usize>().expect("Invalid number"),
        max_distance: pair.1.parse::<usize>().expect("Invalid number"),
    })
}

#[derive(Debug)]
pub struct RaceStat {
    duration: usize,
    max_distance: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    fn load_file() -> String {
        let mut file = File::open("src/fixtures/day6.txt").unwrap();
        let mut file_contents = String::new();
        let _ = file.read_to_string(&mut file_contents);
        file_contents
    }

    #[test]
    fn day6_file2() {
        let start = std::time::Instant::now();
        let input = load_file();

        let race_stats = get_race_stats2(&input);
        let winning_wait_times = get_winning_wait_times(race_stats);
        let margin_of_error = winning_wait_times
            .iter()
            .fold(1, |acc, win| win.len() * acc);

        assert_eq!(margin_of_error, 38220708usize);

        println!("Process in: {:?}", start.elapsed());
    }

    #[test]
    fn day6_test2() {
        let start = std::time::Instant::now();
        let input = r#"Time:      7  15   30
Distance:  9  40  200"#;

        let race_stat = get_race_stats2(&input);
        let winning_wait_times = get_winning_wait_times(race_stat);
        // println!("Winning waits: {:?}", &winning_wait_times);
        let margin_of_error = winning_wait_times
            .iter()
            .fold(1, |acc, win| win.len() * acc);

        assert_eq!(margin_of_error, 71503usize);

        println!("Process in: {:?}", start.elapsed());
    }

    #[test]
    fn day6_file() {
        let start = std::time::Instant::now();
        let input = load_file();

        let race_stats = get_race_stats(&input);
        let winning_wait_times = get_winning_wait_times(race_stats);
        // println!("Winning waits: {:?}", &winning_wait_times);
        let margin_of_error = winning_wait_times
            .iter()
            .fold(1, |acc, win| win.len() * acc);

        assert_eq!(margin_of_error, 741000usize);

        println!("Process in: {:?}", start.elapsed());
    }

    #[test]
    fn day6_test() {
        let start = std::time::Instant::now();
        let input = r#"Time:      7  15   30
Distance:  9  40  200"#;

        let race_stats = get_race_stats(&input);
        let winning_wait_times = get_winning_wait_times(race_stats);
        // println!("Winning waits: {:?}", &winning_wait_times);
        let margin_of_error = winning_wait_times
            .iter()
            .fold(1, |acc, win| win.len() * acc);

        assert_eq!(margin_of_error, 288usize);

        println!("Process in: {:?}", start.elapsed());
    }
}
