use std::collections::HashSet;

use winnow::ascii::{digit1, line_ending, multispace1 as multispace, space1};
use winnow::combinator::{alt, eof, opt, preceded, repeat};
use winnow::token::{take, take_till};
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

#[derive(PartialEq, Debug)]
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

    // #[test]
    // fn day5_test2() {
    // }

    // #[test]
    // fn day5_file2() {
    // }

    #[test]
    fn day5_file() {
        let start = std::time::Instant::now();
        let input = load_file();

        let answer = parse_mapping_file.parse_next(&mut input.as_ref()).unwrap();

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
