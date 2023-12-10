use std::collections::HashSet;

use winnow::ascii::{digit1, line_ending, multispace1 as multispace};
use winnow::combinator::{alt, eof, opt, preceded, repeat};
use winnow::token::take_till;
use winnow::{PResult, Parser};

pub fn card_line_prefix<'a>(input: &mut &'a str) -> PResult<()> {
    let _ = "Card".parse_next(input)?;
    let _ = take_till(0.., |w| w == ':').parse_next(input)?;
    let _ = ':'.parse_next(input)?;

    Ok(())
}

pub fn number<'a>(input: &mut &'a str) -> PResult<usize> {
    let _ = opt(multispace).parse_next(input)?;
    digit1.parse_to().parse_next(input)
}

pub fn numbers<'a>(input: &mut &'a str) -> PResult<Vec<usize>> {
    repeat(0.., number).parse_next(input)
}

pub fn bar_sep<'a>(input: &mut &'a str) -> PResult<()> {
    let _ = multispace.parse_next(input)?;
    '|'.parse_next(input)?;
    let _ = multispace.parse_next(input)?;

    Ok(())
}

pub fn card_line<'a>(input: &mut &'a str) -> PResult<(HashSet<usize>, Vec<usize>)> {
    let winners = preceded(card_line_prefix, numbers).parse_next(input)?;
    let _ = bar_sep.parse_next(input)?;
    let ours = numbers.parse_next(input)?;
    let _ = alt((eof, line_ending)).parse_next(input)?;

    let hs = HashSet::from_iter(winners);
    let data = (hs, ours);

    Ok(data)
}

pub fn parse_stack<'a>(input: &mut &'a str) -> PResult<Vec<(HashSet<usize>, Vec<usize>)>> {
    repeat(0.., card_line).parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    #[derive(Debug)]
    struct CountData {
        card_score: usize,
        copies: usize,
    }

    fn load_file() -> String {
        let mut file = File::open("src/fixtures/day4.txt").unwrap();
        let mut file_contents = String::new();
        let _ = file.read_to_string(&mut file_contents);

        file_contents
    }

    #[test]
    fn day4_test2() {
        let start = std::time::Instant::now();

        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

        let card_stack = parse_stack.parse_next(&mut input.as_ref()).unwrap();

        let mut waterfall_count: Vec<CountData> = card_stack
            .iter()
            .map(|(winners, ours)| {
                let card_score = ours.iter().filter(|o| winners.contains(&o)).count();

                CountData {
                    card_score,
                    copies: 1,
                }
            })
            .collect();

        for i in 0..waterfall_count.len() {
            let CountData { copies, card_score } = waterfall_count[i];

            for _ in 0..copies {
                for ix in 1..=card_score {
                    if i + ix < card_stack.len() {
                        waterfall_count[i + ix].copies += 1;
                    }
                }
            }
        }

        let sum: usize = waterfall_count.iter().map(|c| c.copies).sum();
        assert_eq!(sum, 30usize);

        println!("Process in: {:?}", start.elapsed());
    }

    #[test]
    fn day4_file2() {
        let start = std::time::Instant::now();
        let input = load_file();

        let card_stack = parse_stack.parse_next(&mut input.as_ref()).unwrap();

        let mut waterfall_count: Vec<CountData> = card_stack
            .iter()
            .map(|(winners, ours)| {
                let card_score = ours.iter().filter(|o| winners.contains(&o)).count();

                CountData {
                    card_score,
                    copies: 1,
                }
            })
            .collect();

        for i in 0..waterfall_count.len() {
            let CountData { copies, card_score } = waterfall_count[i];

            for _ in 0..copies {
                for ix in 1..=card_score {
                    if i + ix < card_stack.len() {
                        waterfall_count[i + ix].copies += 1;
                    }
                }
            }
        }

        let sum: usize = waterfall_count.iter().map(|c| c.copies).sum();
        assert_eq!(sum, 14624680usize);

        println!("Process in: {:?}", start.elapsed());
    }

    #[test]
    fn day4_file() {
        let start = std::time::Instant::now();
        let input = load_file();

        let answer = parse_stack.parse_next(&mut input.as_ref()).unwrap();

        let sum = answer
            .iter()
            // .inspect(|x| println!("outer_iter: {:?}", x))
            .fold(0u32, |mut acc, (winners, ours)| {
                let card_score = ours
                    .iter()
                    .filter(|o| winners.contains(&o))
                    .enumerate()
                    // .inspect(|x| println!("inner_iter: {:?}", x))
                    .fold(0u32, |mut acc_inner, (i, _)| {
                        if i == 0 {
                            acc_inner = 1;
                        } else {
                            acc_inner = acc_inner * 2;
                        }
                        acc_inner
                    });

                acc += card_score;
                acc
            });

        assert_eq!(sum, 32609u32);

        println!("Process in: {:?}", start.elapsed());
    }

    #[test]
    fn day4_test() {
        let start = std::time::Instant::now();

        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

        let answer = parse_stack.parse_next(&mut input.as_ref()).unwrap();

        let sum = answer
            .iter()
            .inspect(|x| println!("outer_iter: {:?}", x))
            .fold(0u32, |mut acc, (winners, ours)| {
                let card_score = ours
                    .iter()
                    .filter(|o| winners.contains(&o))
                    .enumerate()
                    .inspect(|x| println!("inner_iter: {:?}", x))
                    .fold(0u32, |mut acc_inner, (i, _)| {
                        if i == 0 {
                            acc_inner = 1;
                        } else {
                            acc_inner = acc_inner * 2;
                        }
                        acc_inner
                    });

                acc += card_score;
                acc
            });

        assert_eq!(sum, 13u32);

        println!("Process in: {:?}", start.elapsed());
    }
}
