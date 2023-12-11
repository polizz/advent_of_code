use winnow::ascii::{digit1, line_ending};
use winnow::combinator::{eof, opt, repeat};
use winnow::token::take;
use winnow::{PResult, Parser};

pub fn file_head_line<'a>(input: &mut &'a str) -> PResult<usize> {
    let _ = take(1usize).parse_next(input)?;
    digit1.parse_to().parse_next(input)
}

pub fn file_header<'a>(input: &mut &'a str) -> PResult<Vec<usize>> {
    let _ = "seeds:".parse_next(input)?;
    let header = repeat(1.., file_head_line).parse_next(input)?;
    let _ = take(2usize).parse_next(input)?;

    Ok(header)
}

pub fn map_line_entry<'a>(input: &mut &'a str) -> PResult<Mapping> {
    let dest_start = digit1.parse_to().parse_next(input)?;
    let _ = take(1usize).parse_next(input)?;
    let src_start = digit1.parse_to().parse_next(input)?;
    let _ = take(1usize).parse_next(input)?;
    let range = digit1.parse_to().parse_next(input)?;
    let _ = opt(line_ending).parse_next(input)?;

    Ok(Mapping {
        dest_start,
        src_start,
        range,
    })
}

pub fn map_hunk<'a>(header_text: &str, input: &mut &'a str) -> PResult<Vec<Mapping>> {
    let _ = (header_text, line_ending).parse_next(input)?;
    let v = repeat(1.., map_line_entry).parse_next(input)?;
    let _ = line_ending.parse_next(input)?;

    Ok(v)
}

pub fn humidity_loc_map<'a>(input: &mut &'a str) -> PResult<Vec<Mapping>> {
    let _ = ("humidity-to-location map:", line_ending).parse_next(input)?;
    repeat(1.., map_line_entry).parse_next(input)
}

pub fn parse_mapping_file<'a>(input: &mut &'a str) -> PResult<MappingSet> {
    let seeds = file_header.parse_next(input)?;

    let seed_soil_map = map_hunk("seed-to-soil map:", input)?;
    let soil_fertilizer_map = map_hunk("soil-to-fertilizer map:", input)?;
    let fertilizer_water_map = map_hunk("fertilizer-to-water map:", input)?;
    let water_light_map = map_hunk("water-to-light map:", input)?;
    let light_temp_map = map_hunk("light-to-temperature map:", input)?;
    let temp_humidity_map = map_hunk("temperature-to-humidity map:", input)?;
    let humidity_loc_map = humidity_loc_map.parse_next(input)?;

    let _ = eof.parse_next(input)?;

    Ok((
        seeds,
        seed_soil_map,
        soil_fertilizer_map,
        fertilizer_water_map,
        water_light_map,
        light_temp_map,
        temp_humidity_map,
        humidity_loc_map,
    ))
}

pub fn get_next_id(map: &Vec<Mapping>, id_link: IdLink) -> IdLink {
    let (id, loc) = id_link;
    let next_range = map
        .iter()
        .find(|t| id >= t.dest_start && id < (t.dest_start + t.range));

    next_range.map_or((id, loc), |r| (r.src_start + id - r.dest_start, loc))
}

pub type IdLink = (usize, usize);

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct Mapping {
    dest_start: usize,
    src_start: usize,
    range: usize,
}

type MappingSet = (
    Vec<usize>,
    Vec<Mapping>,
    Vec<Mapping>,
    Vec<Mapping>,
    Vec<Mapping>,
    Vec<Mapping>,
    Vec<Mapping>,
    Vec<Mapping>,
);

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    fn load_file() -> String {
        let mut file = File::open("src/fixtures/day5.txt").unwrap();
        let mut file_contents = String::new();
        let _ = file.read_to_string(&mut file_contents);
        file_contents
    }

    #[test]
    fn day5_file2() {
        let start = std::time::Instant::now();
        let input = load_file();

        let answer = parse_mapping_file.parse_next(&mut input.as_ref()).unwrap();
        let (seeds, seed_map, soil_map, fert_map, wate_map, ligh_map, temp_map, humi_map) = answer;

        let min_loc = (0..usize::MAX)
            .map(|loc| get_next_id(&humi_map, (loc, loc)))
            .map(|h_link| get_next_id(&temp_map, h_link))
            .map(|t_link| get_next_id(&ligh_map, t_link))
            .map(|w_link| get_next_id(&wate_map, w_link))
            .map(|f_link| get_next_id(&fert_map, f_link))
            .map(|s_link| get_next_id(&soil_map, s_link))
            .map(|seed_link| get_next_id(&seed_map, seed_link))
            .find(|&seed| {
                for i in (0..seeds.len()).step_by(2) {
                    let start = seeds[i];
                    let range = seeds[i + 1];
                    let (seed, loc) = seed;

                    if seed >= start && seed < (start + range) {
                        println!(
                            "Found seed at start: {} range: {} for loc: {}",
                            start, range, loc
                        );
                        return true;
                    }
                }
                false
            });

        assert_eq!(min_loc.unwrap().1, 69323688usize);
        println!("Process in: {:?}", start.elapsed());
    }

    #[test]
    fn day5_test2() {
        let start = std::time::Instant::now();

        let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

        let answer = parse_mapping_file.parse_next(&mut input.as_ref()).unwrap();
        let (seeds, seed_map, soil_map, fert_map, wate_map, ligh_map, temp_map, humi_map) = answer;

        // line: dest-map-start src-map-start range-len
        // 1. take low_loc and synthesize range in order: (low_loc .. low_loc + range)
        // 2. for each loc, try to trace all the way to the top and find a valid seed range
        // 3. Calculate uplink with: curr_loc + src-map-start
        // 4. all uplink range tests are: (dest-map-start .. dest-map-start + range)
        // 5.   if it is within one range, your next uplink is: (src-map-start + range-len - testing-val)
        // 6.   if not within any, your output no IS the same as your input testing no
        // 7. At the top, you must however, match to a seed bin. You do not get to keep the same
        //    numbers you tested with.

        let min_loc = (0..usize::MAX)
            // .inspect(|x| println!("Starting loc: {:#?}", x))
            .map(|loc| get_next_id(&humi_map, (loc, loc)))
            .map(|h_link: IdLink| get_next_id(&temp_map, h_link))
            // .inspect(|x| println!("After temp map: {:#?}", x))
            .map(|t_link| get_next_id(&ligh_map, t_link))
            // .inspect(|x| println!("After light map: {:#?}", x))
            .map(|w_link| get_next_id(&wate_map, w_link))
            // .inspect(|x| println!("After water map: {:#?}", x))
            .map(|f_link| get_next_id(&fert_map, f_link))
            // .inspect(|x| println!("After fert map: {:#?}", x))
            .map(|s_link| get_next_id(&soil_map, s_link))
            // .inspect(|x| println!("After soil map: {:#?}", x))
            .map(|seed_link| get_next_id(&seed_map, seed_link))
            // .inspect(|x| println!("After seed map: {:#?}", x))
            .find(|&seed| {
                for i in (0..seeds.len()).step_by(2) {
                    let start = seeds[i];
                    let range = seeds[i + 1];
                    let (seed, loc) = seed;

                    if seed >= start && seed < (start + range) {
                        println!(
                            "Found seed at start: {} range: {} for loc: {}",
                            start, range, loc
                        );
                        return true;
                    }
                }
                false
            });

        assert_eq!(min_loc.unwrap().1, 46usize);
        println!("Process in: {:?}", start.elapsed());
    }

    #[test]
    fn day5_file() {
        let start = std::time::Instant::now();
        let input = load_file();

        let answer = parse_mapping_file.parse_next(&mut input.as_ref()).unwrap();

        let (seeds, seed_map, soil_map, fert_map, wate_map, ligh_map, temp_map, humi_map) = answer;

        let locations = seeds
            .iter()
            .map(|&seed| {
                let range = seed_map
                    .iter()
                    .find(|ss| seed >= ss.src_start && seed < (ss.src_start + ss.range));

                range.map_or(seed, |r| r.dest_start + seed - r.src_start)
            })
            .map(|soil| {
                let range = soil_map
                    .iter()
                    .find(|ss| soil >= ss.src_start && soil < (ss.src_start + ss.range));

                range.map_or(soil, |r| r.dest_start + soil - r.src_start)
            })
            .map(|fert| {
                let range = fert_map
                    .iter()
                    .find(|ss| fert >= ss.src_start && fert < (ss.src_start + ss.range));

                range.map_or(fert, |r| r.dest_start + fert - r.src_start)
            })
            .map(|wate| {
                let range = wate_map
                    .iter()
                    .find(|ss| wate >= ss.src_start && wate < (ss.src_start + ss.range));

                range.map_or(wate, |r| r.dest_start + wate - r.src_start)
            })
            .map(|ligh| {
                let range = ligh_map
                    .iter()
                    .find(|ss| ligh >= ss.src_start && ligh < (ss.src_start + ss.range));

                range.map_or(ligh, |r| r.dest_start + ligh - r.src_start)
            })
            .map(|temp| {
                let range = temp_map
                    .iter()
                    .find(|ss| temp >= ss.src_start && temp < (ss.src_start + ss.range));

                range.map_or(temp, |r| r.dest_start + temp - r.src_start)
            })
            .map(|humi| {
                let range = humi_map
                    .iter()
                    .find(|ss| humi >= ss.src_start && humi < (ss.src_start + ss.range));

                range.map_or(humi, |r| r.dest_start + humi - r.src_start)
            })
            .collect::<Vec<usize>>();

        let min_seed_location = seeds
            .into_iter()
            .zip(locations.into_iter())
            .min_by(|seed_loc1, seed_loc2| seed_loc1.1.cmp(&seed_loc2.1))
            .unwrap();

        println!("Min Location: {:?}", min_seed_location);
        assert_eq!(min_seed_location.1, 111627841usize);

        println!("Process in: {:?}", start.elapsed());
    }

    #[test]
    fn day5_test() {
        let start = std::time::Instant::now();

        let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

        let answer = parse_mapping_file.parse_next(&mut input.as_ref()).unwrap();
        // line: dest-map-start src-map-start range-len
        //
        // 1. take input_no# and test which range it is within: (src-map-start .. src-map-start + range-len)
        // 2. if it is within one, your output no# is: dest-map-start + (input_no# - src-map-start)
        // 3, if not within any, your output no# IS the same as your seed no#

        let (seeds, seed_map, soil_map, fert_map, wate_map, ligh_map, temp_map, humi_map) = answer;

        // seed_map.sort();

        let locations = seeds
            .iter()
            .map(|&seed| {
                let range = seed_map
                    .iter()
                    .find(|ss| seed >= ss.src_start && seed < (ss.src_start + ss.range));

                range.map_or(seed, |r| r.dest_start + seed - r.src_start)
            })
            .map(|soil| {
                let range = soil_map
                    .iter()
                    .find(|ss| soil >= ss.src_start && soil < (ss.src_start + ss.range));

                range.map_or(soil, |r| r.dest_start + soil - r.src_start)
            })
            .map(|fert| {
                let range = fert_map
                    .iter()
                    .find(|ss| fert >= ss.src_start && fert < (ss.src_start + ss.range));

                range.map_or(fert, |r| r.dest_start + fert - r.src_start)
            })
            .map(|wate| {
                let range = wate_map
                    .iter()
                    .find(|ss| wate >= ss.src_start && wate < (ss.src_start + ss.range));

                range.map_or(wate, |r| r.dest_start + wate - r.src_start)
            })
            .map(|ligh| {
                let range = ligh_map
                    .iter()
                    .find(|ss| ligh >= ss.src_start && ligh < (ss.src_start + ss.range));

                range.map_or(ligh, |r| r.dest_start + ligh - r.src_start)
            })
            .map(|temp| {
                let range = temp_map
                    .iter()
                    .find(|ss| temp >= ss.src_start && temp < (ss.src_start + ss.range));

                range.map_or(temp, |r| r.dest_start + temp - r.src_start)
            })
            .map(|humi| {
                let range = humi_map
                    .iter()
                    .find(|ss| humi >= ss.src_start && humi < (ss.src_start + ss.range));

                range.map_or(humi, |r| r.dest_start + humi - r.src_start)
            })
            .collect::<Vec<usize>>();

        let min_seed_location = seeds
            .into_iter()
            .zip(locations.into_iter())
            .min_by(|seed_loc1, seed_loc2| seed_loc1.1.cmp(&seed_loc2.1))
            .unwrap();

        println!("Min Location: {:?}", min_seed_location);
        assert_eq!(min_seed_location.1, 35usize);

        println!("Process in: {:?}", start.elapsed());
    }
}
