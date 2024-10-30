// --- Day 13: Point of Incidence ---
// With your help, the hot springs team locates an appropriate spring which launches you neatly and precisely up to the edge of Lava Island.
//
// There's just one problem: you don't see any lava.
//
// You do see a lot of ash and igneous rock; there are even what look like gray mountains scattered around. After a while, you make your way to a nearby cluster of mountains only to discover that the valley between them is completely full of large mirrors. Most of the mirrors seem to be aligned in a consistent way; perhaps you should head in that direction?
//
// As you move through the valley of mirrors, you find that several of them have fallen from the large metal frames keeping them in place. The mirrors are extremely flat and shiny, and many of the fallen mirrors have lodged into the ash at strange angles. Because the terrain is all one color, it's hard to tell where it's safe to walk or where you're about to run into a mirror.
//
// You note down the patterns of ash (.) and rocks (#) that you see as you walk (your puzzle input); perhaps by carefully analyzing these patterns, you can figure out where the mirrors are!
//
// For example:
//
// #.##..##.
// ..#.##.#.
// ##......#
// ##......#
// ..#.##.#.
// ..##..##.
// #.#.##.#.
//
// #...##..#
// #....#..#
// ..##..###
// #####.##.
// #####.##.
// ..##..###
// #....#..#
// To find the reflection in each pattern, you need to find a perfect reflection across either a horizontal line between two rows or across a vertical line between two columns.
//
// In the first pattern, the reflection is across a vertical line between two columns; arrows on each of the two columns point at the line between the columns:
//
// 123456789
//     ><
// #.##..##.
// ..#.##.#.
// ##......#
// ##......#
// ..#.##.#.
// ..##..##.
// #.#.##.#.
//     ><
// 123456789
//
//
//  In this pattern, the line of reflection is the vertical line between columns 5 and 6. Because the vertical line is not perfectly in the middle of the pattern, part of the pattern (column 1) has nowhere to reflect onto and can be ignored; every other column has a reflected column within the pattern and must match exactly: column 2 matches column 9, column 3 matches 8, 4 matches 7, and 5 matches 6.
//
//  The second pattern reflects across a horizontal line instead:
//
//  1 #...##..# 1
//  2 #....#..# 2
//  3 ..##..### 3
//  4v#####.##.v4
//  5^#####.##.^5
//  6 ..##..### 6
//  7 #....#..# 7
//
//
// This pattern reflects across the horizontal line between rows 4 and 5. Row 1 would reflect with a hypothetical row 8, but since that's not in the pattern, row 1 doesn't need to match anything. The remaining rows match: row 2 matches row 7, row 3 matches row 6, and row 4 matches row 5.
//
// To summarize your pattern notes, add up the number of columns to the left of each vertical line of reflection; to that, also add 100 multiplied by the number of rows above each horizontal line of reflection. In the above example, the first pattern's vertical line has 5 columns to its left and the second pattern's horizontal line has 4 rows above it, a total of 405.

// --- Part Two ---
// You resume walking through the valley of mirrors and - SMACK! - run directly into one. Hopefully nobody was watching, because that must have been pretty embarrassing.
//
// Upon closer inspection, you discover that every mirror has exactly one smudge: exactly one . or # should be the opposite type.
//
// In each pattern, you'll need to locate and fix the smudge that causes a different reflection line to be valid. (The old reflection line won't necessarily continue being valid after the smudge is fixed.)
//
// Here's the above example again:
//
// #.##..##.
// ..#.##.#.
// ##......#
// ##......#
// ..#.##.#.
// ..##..##.
// #.#.##.#.
//
// #...##..#
// #....#..#
// ..##..###
// #####.##.
// #####.##.
// ..##..###
// #....#..#
// The first pattern's smudge is in the top-left corner. If the top-left # were instead ., it would have a different, horizontal line of reflection:
//
// 1 ..##..##. 1
// 2 ..#.##.#. 2
// 3v##......#v3
// 4^##......#^4
// 5 ..#.##.#. 5
// 6 ..##..##. 6
// 7 #.#.##.#. 7
// With the smudge in the top-left corner repaired, a new horizontal line of reflection between rows 3 and 4 now exists. Row 7 has no corresponding reflected row and can be ignored, but every other row matches exactly: row 1 matches row 6, row 2 matches row 5, and row 3 matches row 4.
//
// In the second pattern, the smudge can be fixed by changing the fifth symbol on row 2 from . to #:
//
// 1v#...##..#v1  #....#..#
// 2^#...##..#^2  #....#..#
// 3 ..##..### 3  ..##..###
// 4 #####.##. 4  #####.##.
// 5 #####.##. 5  #####.##.
// 6 ..##..### 6  ..##..###
// 7 #....#..# 7  #....#..#
// Now, the pattern has a different horizontal line of reflection between rows 1 and 2.
//
// Summarize your notes as before, but instead use the new different reflection lines. In this example, the first pattern's new horizontal line has 3 rows above it and the second pattern's new horizontal line has 1 row above it, summarizing to the value 400.
//
// In each pattern, fix the smudge and find the different line of reflection. What number do you get after summarizing the new reflection line in each pattern in your notes?

use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Eq, PartialEq, Clone, Debug)]
enum MirrorOrientation {
    Horz,
    Vert,
}
type MirrorDetail = (MirrorOrientation, usize);

fn solve_smudges(input: &str) -> usize {
    let blocks: Vec<&str> = input.split("\n\n").collect();
    let blocks: Vec<Vec<&str>> = blocks.iter().map(|block| block.lines().collect()).collect();
    let mut sum = 0usize;

    let flip = |test: char| -> &str {
        if test == '#' {
            return ".";
        }
        return "#";
    };

    'block_iter: for block in blocks.iter() {
        let (max_row, max_col) = (block.len().clone(), block[0].len().clone());
        let def_horz_hashes = make_horz_hash_grids(block);
        let def_vert_hashes = make_vert_hash_grids(block);
        let selected_def = select_mirror_block(&def_horz_hashes, &def_vert_hashes, &None);

        for ir in 0..max_row {
            for ic in 0..max_col {
                let test_position: Vec<char> = block[ir]
                    .char_indices()
                    .filter(|(i, _)| i == &ic)
                    .map(|c| c.1)
                    .take(1)
                    .collect();
                debug_assert!(test_position.len() == 1);

                let flipped_position = flip(test_position[0]);
                let try_line = format!(
                    "{}{}{}",
                    block[ir][0..ic].to_owned(),
                    flipped_position,
                    block[ir][ic + 1..].to_owned()
                );

                let try_block = {
                    let (first, last) = block.split_at(ir);
                    let mut v = Vec::with_capacity(max_row);
                    v.extend(
                        first
                            .iter()
                            .map(|s| (*s).to_owned())
                            .collect::<Vec<String>>(),
                    );
                    v.push(try_line.clone());
                    v.extend(
                        last[1..]
                            .iter()
                            .map(|s| (*s).to_owned())
                            .collect::<Vec<String>>(),
                    );
                    v
                };

                let ref_block = try_block.iter().map(|l| &l[..]).collect();
                let horz_hash_try = make_horz_hash_grids(&ref_block);
                let vert_hash_try: Vec<u64> = make_vert_hash_grids(&ref_block);

                let try_mirror_block: Option<(MirrorOrientation, usize)> =
                    select_mirror_block(&horz_hash_try, &vert_hash_try, &selected_def);

                if let Some(try_mirror) = try_mirror_block.clone() {
                    let selected_def = selected_def.clone().unwrap();
                    if try_mirror != selected_def {
                        sum += try_mirror.1;
                        continue 'block_iter;
                    }
                }
            }
        }

        let (_, selected_def_idx) = selected_def.unwrap();
        sum += selected_def_idx;
    }

    sum
}

#[inline(always)]
fn make_horz_hash_grids(lines: &Vec<&str>) -> Vec<u64> {
    lines
        .iter()
        .map(|&line| {
            let mut hasher = DefaultHasher::new();
            line.hash(&mut hasher);
            hasher.finish()
        })
        .collect()
}

#[inline(always)]
fn make_vert_hash_grids(lines: &Vec<&str>) -> Vec<u64> {
    let mut column_hashes: Vec<DefaultHasher> = vec![DefaultHasher::new(); lines[0].len()];

    lines.iter().for_each(|&line| {
        for (ix, c) in line.chars().enumerate() {
            c.hash(&mut column_hashes[ix]);
        }
    });

    let ver_hashes: Vec<u64> = column_hashes
        .iter()
        .map(|h: &DefaultHasher| h.finish())
        .collect();

    ver_hashes
}

#[inline(always)]
fn make_horz_hash_grids_from_lines(input: &str) -> Vec<Vec<u64>> {
    let mut all_hor_blocks = vec![];
    let mut line_block = vec![];

    for l in input.lines() {
        if l.trim().len() > 0 {
            let mut hasher = DefaultHasher::new();
            l.hash(&mut hasher);

            line_block.push(hasher.finish());
        } else {
            all_hor_blocks.push(line_block);
            line_block = vec![];
        }
    }
    all_hor_blocks.push(line_block);
    all_hor_blocks
}

#[inline(always)]
fn make_vert_hash_grids_from_lines(input: &str) -> Vec<Vec<u64>> {
    let mut vertical_hashers: Vec<Vec<DefaultHasher>> = vec![];
    let mut init_block = true;
    let mut vert_block_id = 0usize;
    for line in input.lines() {
        if init_block {
            vertical_hashers.push(vec![]);
            for _ in 0..line.len() {
                vertical_hashers[vert_block_id].push(DefaultHasher::new());
            }
        }
        init_block = false;

        if line.trim().len() > 0 {
            for (ix, c) in line.chars().enumerate() {
                c.hash(&mut vertical_hashers[vert_block_id][ix]);
            }
        } else {
            vert_block_id += 1;
            init_block = true;
        }
    }
    let all_ver_blocks: Vec<Vec<u64>> = vertical_hashers
        .iter()
        .map(|v| v.iter().map(|h| h.finish()).collect())
        .collect();

    all_ver_blocks
}

#[inline(always)]
fn ident_mirror_and_idx(block: &Vec<u64>, skip_idx: Option<usize>) -> Option<usize> {
    let mut mirror_stack: Vec<u64> = vec![];
    let mut prev_line: Option<u64> = None;
    let mut mirror_check = false;
    let mut mirror_idx: Option<usize> = None;
    let mut check_idx: Option<usize> = None;

    for (ix, curr_line) in block.iter().enumerate() {
        if !mirror_check {
            if prev_line
                .is_some_and(|p| p == *curr_line && !skip_idx.is_some_and(|skip| skip == ix))
            {
                mirror_idx = Some(ix);
                if ix > 1 {
                    check_idx = Some(ix - 2);
                } else {
                    check_idx = None;
                }
                mirror_check = true;
            }
            mirror_stack.push(curr_line.clone());
            prev_line = Some(curr_line.clone());
        } else {
            if let Some(cidx) = check_idx {
                if mirror_stack.get(cidx).is_some_and(|p| p == curr_line) {
                    if check_idx.is_some_and(|c| c > 0) {
                        check_idx = Some(check_idx.expect("valid") - 1);
                    } else {
                        check_idx = None;
                    }
                } else {
                    mirror_check = false;
                    mirror_idx = None;
                }
            }
            mirror_stack.push(curr_line.clone());
            prev_line = Some(curr_line.clone());
        }
    }
    mirror_idx
}

fn select_mirror_block(
    horz_block: &Vec<u64>,
    vert_block: &Vec<u64>,
    skip_idx: &Option<(MirrorOrientation, usize)>,
) -> Option<(MirrorOrientation, usize)> {
    let h_skip = skip_idx
        .clone()
        .filter(|(dir, _)| *dir == MirrorOrientation::Horz)
        .map(|(_, skip_idx)| skip_idx / 100);

    let h_mirror_idx =
        ident_mirror_and_idx(&horz_block, h_skip).map(|h| (MirrorOrientation::Horz, h * 100));

    let v_skip = skip_idx
        .clone()
        .filter(|(dir, _)| *dir == MirrorOrientation::Vert)
        .map(|(_, skip_idx)| skip_idx);

    let v_mirror_idx =
        ident_mirror_and_idx(&vert_block, v_skip).map(|v| (MirrorOrientation::Vert, v));

    h_mirror_idx.or(v_mirror_idx)
}

fn select_mirror_blocks(
    hor_blocks: Vec<Vec<u64>>,
    ver_blocks: Vec<Vec<u64>>,
) -> impl Iterator<Item = Option<(MirrorOrientation, usize)>> {
    hor_blocks
        .into_iter()
        .zip(ver_blocks)
        .map(|(horz_block, vert_block)| select_mirror_block(&horz_block, &vert_block, &None))
}

fn solve(input: &str) -> usize {
    let all_hor_blocks = make_horz_hash_grids_from_lines(input);
    let all_ver_blocks: Vec<Vec<u64>> = make_vert_hash_grids_from_lines(input);

    let mirror_blocks: Vec<usize> = select_mirror_blocks(all_hor_blocks, all_ver_blocks)
        .filter(|b| b.is_some())
        .map(|b| b.unwrap())
        .map(|(_, idx)| idx)
        .collect();
    mirror_blocks.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day13_part2_file() {
        let start = std::time::Instant::now();

        let input = std::str::from_utf8(include_bytes!("fixtures/day13.txt")).expect("valid str");
        let result = solve_smudges(input);

        assert_eq!(result, 31974);
        println!("Process in: {:?}", start.elapsed());
    }

    #[test]
    fn day13_part2_issue2() {
        let input = r#"####..#..#..#
.##.#..#...#.
#..#.#....#.#
....########.
#..#.#.##.#.#
....##.##.##.
.....#....#.."#;
        let result = solve_smudges(input);
        assert_eq!(result, 8);
    }

    #[test]
    fn day13_part2_issue1() {
        let input = r#"..##....#
..##....#
#.#......
..##.##.#
#...#..#.
..####.##
.###....#"#;
        let result = solve_smudges(input);
        assert_eq!(result, 6);
    }

    #[test]
    fn day13_part2_sample() {
        let input = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;
        let result = solve_smudges(input);
        assert_eq!(result, 400);
    }

    #[test]
    fn day13_part1_file() {
        let start = std::time::Instant::now();

        let input = std::str::from_utf8(include_bytes!("fixtures/day13.txt")).expect("valid str");
        let result = solve(input);

        assert_eq!(result, 35210);
        println!("Process in: {:?}", start.elapsed());
    }

    #[test]
    fn day13_part1_custom_6() {
        let input = r#"
##...#..#...#
..##.#..#.##.
...###..###..
..#..#..#..#.
..###.##.###.
..###...####.
#####....####"#;

        let result = solve(input);
        assert_eq!(result, 1);
    }

    #[test]
    fn day13_part1_custom2() {
        let input = r#"....#...####..#
.###.....####.#
###.#.##..#.#..
#.#.######...#.
#.#####.#..#.#.
...#......#.##.
####....#......
#.#..#..#####.#
##..#.##...#.##
...#.#.###.###.
..####.##...#.#
..####.##...#.#
...#.#..##.###.
...#.#..##.###.
..####.##...#.#
..####.##...#.#
...#.#.###.###."#;

        let result = solve(input);
        assert_eq!(result, 1300);
    }

    #[test]
    fn day13_part1_custom1() {
        let input = r#"#...#.#####
##.#..#.##.
##.........
#..#...#.##
..#.##.#...
##..#.###..
##..#.###.."#;

        let result = solve(input);
        assert_eq!(result, 600);
    }

    #[test]
    fn day13_part1_sample() {
        let input = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.
         
#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;
        let result = solve(input);
        assert_eq!(result, 405);
    }
}
