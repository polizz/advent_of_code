// --- Day 12: Hot Springs ---
// You finally reach the hot springs! You can see steam rising from secluded areas attached to the primary, ornate building.
//
// As you turn to enter, the researcher stops you. "Wait - I thought you were looking for the hot springs, weren't you?" You indicate that this definitely looks like hot springs to you.
//
// "Oh, sorry, common mistake! This is actually the onsen! The hot springs are next door."
//
// You look in the direction the researcher is pointing and suddenly notice the massive metal helixes towering overhead. "This way!"
//
// It only takes you a few more steps to reach the main gate of the massive fenced-off area containing the springs. You go through the gate and into a small administrative building.
//
// "Hello! What brings you to the hot springs today? Sorry they're not very hot right now; we're having a lava shortage at the moment." You ask about the missing machine parts for Desert Island.
//
// "Oh, all of Gear Island is currently offline! Nothing is being manufactured at the moment, not until we get more lava to heat our forges. And our springs. The springs aren't very springy unless they're hot!"
//
// "Say, could you go up and see why the lava stopped flowing? The springs are too cold for normal operation, but we should be able to find one springy enough to launch you up there!"
//
// There's just one problem - many of the springs have fallen into disrepair, so they're not actually sure which springs would even be safe to use! Worse yet, their condition records of which springs are damaged (your puzzle input) are also damaged! You'll need to help them repair the damaged records.
//
// In the giant field just outside, the springs are arranged into rows. For each row, the condition records show every spring and whether it is operational (.) or damaged (#). This is the part of the condition records that is itself damaged; for some springs, it is simply unknown (?) whether the spring is operational or damaged.
//
// However, the engineer that produced the condition records also duplicated some of this information in a different format! After the list of springs for a given row, the size of each contiguous group of damaged springs is listed in the order those groups appear in the row. This list always accounts for every damaged spring, and each number is the entire size of its contiguous group (that is, groups are always separated by at least one operational spring: #### would always be 4, never 2,2).
//
// So, condition records with no unknown spring conditions might look like this:
//
// #.#.### 1,1,3
// .#...#....###. 1,1,3
// .#.###.#.###### 1,3,1,6
// ####.#...#... 4,1,1
// #....######..#####. 1,6,5
// .###.##....# 3,2,1
// However, the condition records are partially damaged; some of the springs' conditions are actually unknown (?). For example:
//
// ???.### 1,1,3
// .??..??...?##. 1,1,3
// ?#?#?#?#?#?#?#? 1,3,1,6
// ????.#...#... 4,1,1
// ????.######..#####. 1,6,5
// ?###???????? 3,2,1
// Equipped with this information, it is your job to figure out how many different arrangements of operational and broken springs fit the given criteria in each row.
//
// In the first line (???.### 1,1,3), there is exactly one way separate groups of one, one, and three broken springs (in that order) can appear in that row: the first three unknown springs must be broken, then operational, then broken (#.#), making the whole row #.#.###.
//
// The second line is more interesting: .??..??...?##. 1,1,3 could be a total of four different arrangements. The last ? must always be broken (to satisfy the final contiguous group of three broken springs), and each ?? must hide exactly one of the two broken springs. (Neither ?? could be both broken springs or they would form a single contiguous group of two; if that were true, the numbers afterward would have been 2,3 instead.) Since each ?? can either be #. or .#, there are four possible arrangements of springs.
//
// The last line is actually consistent with ten different arrangements! Because the first number is 3, the first and second ? must both be . (if either were #, the first number would have to be 4 or higher). However, the remaining run of unknown spring conditions have many different ways they could hold groups of two and one broken springs:
//
// ?###???????? 3,2,1
// .###.##.#...
// .###.##..#..
// .###.##...#.
// .###.##....#
// .###..##.#..
// .###..##..#.
// .###..##...#
// .###...##.#.
// .###...##..#
// .###....##.#
// In this example, the number of possible arrangements for each row is:
//
// ???.### 1,1,3 - 1 arrangement
// .??..??...?##. 1,1,3 - 4 arrangements
// ?#?#?#?#?#?#?#? 1,3,1,6 - 1 arrangement
// ????.#...#... 4,1,1 - 1 arrangement
// ????.######..#####. 1,6,5 - 4 arrangements
// ?###???????? 3,2,1 - 10 arrangements
// Adding all of the possible arrangement counts together produces a total of 21 arrangements.
//
// For each row, count all of the different arrangements of operational and broken springs that meet the given criteria. What is the sum of those counts?
//
// --- Part Two ---
// As you look out at the field of springs, you feel like there are way more springs than the condition records list. When you examine the records, you discover that they were actually folded up this whole time!
//
// To unfold the records, on each row, replace the list of spring conditions with five copies of itself (separated by ?) and replace the list of contiguous groups of damaged springs with five copies of itself (separated by ,).
//
// So, this row:
//
// .# 1
// Would become:
//
// .#?.#?.#?.#?.# 1,1,1,1,1
//
// ???.### 1,1,3 - 1 arrangement
// The first line of the above example would become:
//
// ???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3
// In the above example, after unfolding, the number of possible arrangements for some rows is now much larger:
//
// ???.### 1,1,3 - 1 arrangement
// .??..??...?##. 1,1,3 - 16384 arrangements
// ?#?#?#?#?#?#?#? 1,3,1,6 - 1 arrangement
// ????.#...#... 4,1,1 - 16 arrangements
// ????.######..#####. 1,6,5 - 2500 arrangements
// ?###???????? 3,2,1 - 506250 arrangements
// After unfolding, adding all of the possible arrangement counts together produces 525152.
//
// Unfold your condition records; what is the new sum of possible arrangement counts?
#![allow(warnings)]
#[allow(unused)]
#[allow(unused_variables)]
#[allow(unused_assignments)]
use anyhow::Result;
use rayon::prelude::*;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::atomic::AtomicUsize;

#[inline(always)]
fn verify_placement(
    d: usize,
    line: &[u8],
    line_len: usize,
    rl_data: &Vec<usize>,
    rl_len: usize,
    rl_idx: usize,
) -> bool {
    // the position previous the run beginning must allow a period separator
    if d > 0 {
        match line[d - 1] {
            b'.' | b'?' => {}
            _ => {
                return false;
            }
        }
    }

    // the position following the run ending must allow a period separator
    if d + rl_data[rl_idx] <= line_len {
        match line[d + rl_data[rl_idx]] {
            b'.' | b'?' => {}
            _ => {
                return false;
            }
        }
    }

    //  if last run length and have not finished moving through string, need to ensure there are no more unused real "broken" entries
    //  after us
    if rl_idx == rl_len && d + rl_data[rl_idx] < line_len {
        let later_pos = line[d + rl_data[rl_idx]..].iter().position(|c| *c == b'#');
        if later_pos.is_some() {
            return false;
        }
    }

    true
}

#[inline(always)]
fn try_placement(d: usize, line: &[u8], rl_data: &Vec<usize>, rl_idx: usize) -> Option<usize> {
    for r in 0..rl_data[rl_idx] {
        match line[d + r] {
            b'#' | b'?' => {
                continue;
            }
            _ => {
                return Some(r);
            }
        }
    }

    None
}

fn get_count_r<'a, 'b>(
    line: &'a [u8],
    line_len: usize,
    idx: usize,
    rl_data: &'a Vec<usize>,
    rl_len: usize,
    rl_idx: usize,
    depth: usize,
    mut cache: &'b mut HashMap<(&'a str, &'a [usize]), usize>,
) -> usize {
    // we've found an arrangement when all run length indices are used/placed
    if rl_idx > rl_len {
        return 1;
    }
    // if we've run off the end of the data string
    // and we have not used all RL len indices, we have failed to find an arrangement
    if idx > line_len {
        return 0;
    }

    if let Some(r) = cache.get(&(
        std::str::from_utf8(&line[idx..]).unwrap(),
        &rl_data[rl_idx..],
    )) {
        return r.clone();
    };

    let mut total = 0;
    let max_d_idx = line
        .iter()
        .skip(idx)
        .position(|c| *c == b'#')
        .map_or(line_len, |i| i + idx);

    let mut d = idx;
    'outer: while d <= max_d_idx {
        let max_rl_idx = d + rl_data[rl_idx] - 1;

        // do we have enough string left to hold the digit
        if max_rl_idx <= line_len {
            if let Some(r) = try_placement(d, line, rl_data, rl_idx) {
                d = d + 1 + r;
                continue 'outer;
            };

            if !verify_placement(d, line, line_len, rl_data, rl_len, rl_idx) {
                d += 1;
                continue 'outer;
            }

            let r = get_count_r(
                line,
                line_len,
                d + rl_data[rl_idx] + 1,
                rl_data,
                rl_len,
                rl_idx + 1,
                depth + 1,
                &mut cache,
            );
            total += r;
        }
        d += 1;
    }

    cache.insert(
        (
            std::str::from_utf8(&line[idx..]).unwrap(),
            &rl_data[rl_idx..],
        ),
        total,
    );

    // dbg!(&cache.values());

    total
}

fn get_count<'a>(line: &'a str) -> usize {
    let records: Vec<&str> = line.split(" ").collect();
    let (data, rl_data) = (records[0], records[1]);

    let data = data.as_bytes();
    let rl_data: Vec<usize> = rl_data
        .split(",")
        .map(|n| n.parse::<usize>().expect("Input data is numbers"))
        .collect();

    let mut cache = HashMap::<(&'a str, &'a [usize]), usize>::new();
    let line_len = data.len() - 1;

    let s = get_count_r(
        data,
        line_len,
        0,
        &rl_data,
        rl_data.len() - 1,
        0,
        0,
        &mut cache,
    );
    s
}

#[derive(Debug, Clone)]
struct ParentCtx<'a> {
    max_idx: usize,
    total: usize,
    idx: usize,
    rl_idx: usize,
    originating_idx: usize,
    cache_tag: Option<&'a str>,
}

#[derive(Debug, Clone)]
enum Search<'a> {
    Resume(ParentCtx<'a>),
    Init(usize, usize),
    Add(usize),
}

fn get_count_iter<'a>(line: &'a str) -> usize {
    let records: Vec<&str> = line.split(" ").collect();
    let (data, rl_data) = (records[0], records[1]);

    let rl_data: Vec<usize> = rl_data
        .split(",")
        .map(|n| n.parse::<usize>().expect("Input data is numbers"))
        .collect();

    let line = data.as_bytes();
    let line_len = line.len() - 1;
    let rl_len = rl_data.len() - 1;

    let mut cache = HashMap::<(&'a str, &'a [usize]), usize>::new();

    let mut stack = vec![];
    stack.push(Search::Init(0, 0));

    let mut grand_total = 0;

    'top: while let Some(next_search) = stack.pop() {
        let mut org_idx = 0usize;
        let mut idx = 0usize;
        let mut rl_idx = 0usize;
        let mut max_d_idx = line_len;
        let mut total: usize = 0;
        let mut originating_idx = 0usize;
        let mut cache_tag: Option<&str> = None;

        // TODO: "init" calls that return values, can modify the stack end item to add/return their
        //"total" value up the call stack.
        // TODO: "init" calls that are finished scanning through the d index, can also return their total
        //value by modifying their caller
        // via the stack end item, and pass up the final total value.
        // TODO: inserting cache can happen at end of return as well? To get the correct associated
        // string, you must pass the current "d" idx, that is being processed that causes the child
        // "init" call, in the child call data.

        match next_search {
            Search::Init(new_d, new_rl_idx) => {
                org_idx = new_d;
                idx = new_d;
                rl_idx = new_rl_idx;

                // we've found an arrangement when all run length indices are used/placed
                if rl_idx > rl_len {
                    println!("Ret 1");
                    stack.push(Search::Add(1));
                    continue 'top;
                }

                // if we've run off the end of the data string
                // and we have not used all RL len indices, we have failed to find an arrangement
                if idx > line_len {
                    stack.push(Search::Add(0));
                    continue 'top;
                }

                let new_max_idx = line
                    .iter()
                    .skip(idx)
                    .position(|c| *c == b'#')
                    .map_or(line_len, |i| i + idx);
                max_d_idx = new_max_idx;
            }
            Search::Add(value) => {
                if let Some(resume_ctx) = stack.last_mut() {
                    match resume_ctx {
                        Search::Resume(parent_ctx) => {
                            parent_ctx.total += value;
                        }
                        _ => panic!("should always resume after an add op"),
                    }
                }
                let len = stack.len();

                continue 'top;
            }
            Search::Resume(ParentCtx {
                max_idx: prev_max_idx,
                total: prev_total,
                idx: prev_idx,
                rl_idx: prev_rl_idx,
                originating_idx: orig_idx,
                cache_tag: tag,
            }) => {
                max_d_idx = prev_max_idx;
                total = prev_total;
                idx = prev_idx;
                rl_idx = prev_rl_idx;
                originating_idx = orig_idx;
                cache_tag = tag
            }
        };

        if let Some(r) = cache.get(&(
            std::str::from_utf8(&line[idx..]).unwrap(),
            &rl_data[rl_idx..],
        )) {
            stack.push(Search::Add(*r));
            continue 'top;
        };

        let mut d = idx;
        'digit_str: while d <= max_d_idx {
            let max_rl_idx = d + rl_data[rl_idx] - 1;
            // do we even have enough string left to hold this digit?
            if max_rl_idx <= line_len && rl_data[rl_idx] <= line[d..].len() {
                // if entry totally missing, run manual placement and update cache
                if let Some(r) = try_placement(d, line, &rl_data, rl_idx) {
                    d = d + 1 + r;
                    continue 'digit_str;
                }
                if !verify_placement(d, line, line_len, &rl_data, rl_len, rl_idx) {
                    d += 1;
                    continue 'digit_str;
                }
                // restore max_d_idx for when we move to the
                // next d index after our child finishes
                stack.push(Search::Resume(ParentCtx {
                    originating_idx: d,
                    max_idx: max_d_idx,
                    total,
                    idx: d + 1,
                    rl_idx,
                    cache_tag: Some(std::str::from_utf8(&line[d..]).unwrap()),
                }));

                // child process
                stack.push(Search::Init(d + rl_data[rl_idx] + 1, rl_idx + 1));
                continue 'top;
            }
            d += 1;
        }

        if let Some(resume_ctx) = stack.last_mut() {
            match resume_ctx {
                Search::Resume(parent_ctx) => {
                    parent_ctx.total += total;
                }
                _ => {}
            }
        }

        if let Some(_tag) = cache_tag {
            cache.insert(
                (
                    // std::str::from_utf8(&line[originating_idx..]).unwrap(),
                    cache_tag.unwrap_or(std::str::from_utf8(&line).unwrap()),
                    &rl_data[rl_idx..],
                ),
                total,
            );
        }

        grand_total += total;
    }
    dbg!(&cache);

    grand_total
}

fn solve_stream_iter(all_data: &str, unfold: bool) -> Result<usize> {
    let lines = all_data.par_lines();

    let arrangements: Result<usize> = lines
        .map(|line| {
            let line = if unfold {
                unfold_stream(line)?
            } else {
                line.to_owned()
            };

            let cnt = get_count_iter(&line);
            Ok(cnt)
        })
        .sum();

    arrangements
}

fn solve_stream(all_data: &str, unfold: bool) -> Result<usize> {
    let lines = all_data.par_lines();
    let n = AtomicUsize::new(0);

    let arrangements: Result<usize> = lines
        .map(|line| {
            let line = if unfold {
                unfold_stream(line)?
            } else {
                line.to_owned()
            };

            let mut h = std::hash::DefaultHasher::new();
            line.as_bytes().hash(&mut h);
            let cnt = get_count(&line);
            let this_n = n.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            println!("Sum for line {:?}\t\t{:?}\t\t{:?}", this_n, &cnt, &line);

            Ok(cnt)
        })
        .sum();

    arrangements
}

fn unfold_stream(line: &str) -> Result<String> {
    let recs: Vec<&str> = line.split(' ').collect();

    let mut side1 = vec![];
    for _ in 0..5 {
        side1.push(recs[0]);
    }
    let mut side1 = side1.join("?");

    let mut side2 = vec![];
    for _ in 0..5 {
        side2.push(recs[1]);
    }
    let side2 = side2.join(",");

    side1.push(' ');
    side1.push_str(&side2);

    // dbg!(&side1);

    Ok(side1)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn day12_file2_rec() {
        let start = std::time::Instant::now();
        let input = std::str::from_utf8(include_bytes!("fixtures/day12.txt")).expect("valid str");

        let total_min_spans = solve_stream(input, true).unwrap();

        assert_eq!(total_min_spans, 1909291258644usize);
        println!("Process in: {:?}", start.elapsed());
    }

    #[test]
    fn day12_file2_iter() {
        let start = std::time::Instant::now();
        let input = std::str::from_utf8(include_bytes!("fixtures/day12.txt")).expect("valid str");

        let total_min_spans = solve_stream_iter(input, true).unwrap();

        assert_eq!(total_min_spans, 1909291258644usize);
        println!("Process in: {:?}", start.elapsed());
    }

    #[test]
    fn day12_file1_iter() {
        let start = std::time::Instant::now();
        let input = std::str::from_utf8(include_bytes!("fixtures/day12.txt")).expect("valid str");
        // let input = include_bytes!("fixtures/day12.txt");

        let total_min_spans = solve_stream_iter(input, false).unwrap();

        assert_eq!(total_min_spans, 7260usize);
        println!("Process in: {:?}", start.elapsed());
    }

    #[test]
    fn day12_sample1_iter() {
        let start = std::time::Instant::now();
        let input = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;
        // let input = r#"?###???????? 3,2,1"#;

        let input = input;
        let total_min_spans = solve_stream(input, false).unwrap();

        assert_eq!(total_min_spans, 21usize);
        // assert_eq!(total_min_spans, 10usize);
        println!("Process in: {:?}", start.elapsed());
    }

    #[test]
    fn day12_unfold_stream() {
        let input = ".# 1";
        let result = unfold_stream(input).unwrap();
        assert_eq!(result, ".#?.#?.#?.#?.# 1,1,1,1,1");

        let input2 = "???.### 1,1,3";
        let result2 = unfold_stream(input2).unwrap();

        assert_eq!(
            result2,
            "???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3"
        );
    }

    #[test]
    fn day12_part2_file() {
        let start = std::time::Instant::now();
        let input = std::str::from_utf8(include_bytes!("fixtures/day12.txt")).unwrap();

        let result = solve_stream_iter(input, true).unwrap();
        assert_eq!(result, 0usize);
        println!("Process in: {:?}", start.elapsed());
    }

    #[test]
    fn day12_part2_sample() {
        let start = std::time::Instant::now();
        let input = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;

        let result = solve_stream(input, true).unwrap();
        assert_eq!(result, 525152usize);
        println!("Process in: {:?}", start.elapsed());
    }

    #[test]
    fn day12_file1() {
        let start = std::time::Instant::now();
        let input = std::str::from_utf8(include_bytes!("fixtures/day12.txt")).expect("valid str");

        let total_min_spans = solve_stream(input, false).unwrap();

        assert_eq!(total_min_spans, 7260usize);
        println!("Process in: {:?}", start.elapsed());
    }

    #[test]
    fn day12_tiny1() {
        println!("Test1");
        let input = "?###???????? 3,2,1";
        let result = get_count(input);
        assert_eq!(result, 10usize);

        println!("Test2");
        let input = "????.######..#####. 1,6,5";
        let result = get_count(input);
        assert_eq!(result, 4usize);

        println!("Test3");
        let input = "????.#...#... 4,1,1";
        let result = get_count(input);
        assert_eq!(result, 1usize);

        println!("Test4");
        let input = "?#?#?#?#?#?#?#? 1,3,1,6";
        let result = get_count(input);
        assert_eq!(result, 1usize);

        println!("Test5");
        let input = ".??..??...?##. 1,1,3";
        let result = get_count(input);
        assert_eq!(result, 4usize);

        println!("Test6");
        let input = "???.### 1,1,3";
        let result = get_count(input);
        assert_eq!(result, 1usize);
    }

    #[test]
    fn day12_tiny2() {
        println!("Test7");
        let input = "?.???.?#.????#?#??# 1,1,1,1,8";
        let result = get_count_iter(input);
        assert_eq!(result, 5usize);

        println!();
        println!();
        println!();

        println!("Test7_rec");
        let input = "?.???.?#.????#?#??# 1,1,1,1,8";
        let result = get_count(input);
        assert_eq!(result, 5usize);
    }

    #[test]
    fn day12_sample1() {
        let start = std::time::Instant::now();
        let input = r#"?###???????? 3,2,1"#;

        let input = input;
        let total_min_spans = solve_stream(input, false).unwrap();

        assert_eq!(total_min_spans, 10usize);
        println!("Process in: {:?}", start.elapsed());
    }
}
