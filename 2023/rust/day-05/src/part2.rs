use std::collections::HashSet;

use nom::{
    IResult,
    bytes::complete::{tag, take_until},
    character::complete::{multispace0, digit1, multispace1, newline},
    sequence::{tuple, separated_pair}, multi::separated_list1,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[derive(Clone, Copy, Debug)]
struct AlmanacMap {
    dest_start: u64,
    source_start: u64,
    length: u64,
}

impl AlmanacMap {
    fn from(source: &u64) -> AlmanacMap {
        AlmanacMap {
            dest_start: source.clone(),
            source_start: source.clone(),
            length: 1,
        }
    }

    fn contains_source(&self, source: &u64) -> bool {
        let source_end = &self.source_start + &self.length;
        source >= &self.source_start && source < &source_end
    }

    fn convert_source_to_dest(&self, source: u64) -> u64 {
        // Determine if the destination is smaller or bigger than the source.
        let change: i64 = self.source_start.abs_diff(self.dest_start) as i64;
        let change: i64 = change * (if self.dest_start >= self.source_start {
            1
        } else {
            -1
        });

        ((source as i64) + change) as u64
    }
}

fn parse_space_list(input: &str) -> IResult<&str, Vec<u64>> {
    let (_, numbers) = separated_list1(tag(" "), digit1)(input)?;
    let numbers = numbers.into_iter().map(|number| number.parse::<u64>().unwrap()).collect();
    Ok((input, numbers))
}

fn parse_seeds(input: &str) -> IResult<&str, HashSet<u64>> {
    let (input, _) = tuple((tag("seeds:"), multispace0))(input)?;
    let (input, seeds) = separated_list1(multispace1, separated_pair(digit1, multispace1, digit1))(input)?;
    let (input, _) = multispace1(input)?;

    let seeds = seeds
    .into_iter()
    .map(|seed| {
        let start = seed.0.parse::<u64>().expect("a number");
        let length = seed.1.parse::<u64>().expect("a number");
        (start..(start + length)).collect::<Vec<u64>>()
    })
    .flatten()
    .collect::<HashSet<u64>>();

    Ok((input, seeds))
}

fn parse_map(input: &str) -> IResult<&str, Vec<AlmanacMap>> {
    let (input, _) = tuple((take_until(":"), tag(":"), newline))(input)?;
    let maps: Vec<AlmanacMap> = input.lines().map(|line| {
        let (_, seed_map) = parse_space_list(&line).expect("space separated list");
        AlmanacMap {
            dest_start: seed_map[0],
            source_start: seed_map[1],
            length: seed_map[2],
        }
    })
    .collect();
    Ok((input, maps))
}

fn get_almanac_map(seed: u64, seed_maps: &Vec<AlmanacMap>) -> Option<AlmanacMap> {
    let seed_maps: Vec<AlmanacMap> = seed_maps
        .clone() // because we need to make new maps if none were found.
        .into_par_iter()
        .filter(|&seed_map| seed_map.contains_source(&seed))
        .collect();

    Some(match seed_maps.len() {
        0 => AlmanacMap::from(&seed),
        _ => seed_maps[0],
    })
}

fn get_location_from_seed(seed: &u64, seed_maps: &Vec<Vec<AlmanacMap>>) -> u64 {
    let mut seed_maps = seed_maps.iter();

    let seed = seed.clone();
    let seed_map = get_almanac_map(seed, seed_maps.next().unwrap()).expect("seed map can be found");
    //println!("seed: {}, {:?}", &seed, &seed_map);

    let soil = seed_map.convert_source_to_dest(seed);
    let soil_map = get_almanac_map(soil, seed_maps.next().unwrap()).expect("soil map can be found");
    //println!("soil: {}, {:?}", &soil, &soil_map);

    let fertilizer = soil_map.convert_source_to_dest(soil);
    let fertilizer_map = get_almanac_map(fertilizer, seed_maps.next().unwrap()).expect("fertilizer map can be found");
    //println!("fertilizer: {}, {:?}", &fertilizer, &fertilizer_map);

    let water = fertilizer_map.convert_source_to_dest(fertilizer);
    let water_map = get_almanac_map(water, seed_maps.next().unwrap()).expect("water map can be found");
    //println!("water: {}, {:?}", &water, &water_map);

    let light = water_map.convert_source_to_dest(water);
    let light_map = get_almanac_map(light, seed_maps.next().unwrap()).expect("light map can be found");
    //println!("light: {}, {:?}", &light, &light_map);

    let temperature = light_map.convert_source_to_dest(light);
    let temperature_map = get_almanac_map(temperature, seed_maps.next().unwrap()).expect("temperature map can be found");
    //println!("temperature: {}, {:?}", &temperature, &temperature_map);

    let humidity = temperature_map.convert_source_to_dest(temperature);
    let humidity_map = get_almanac_map(humidity, seed_maps.next().unwrap()).expect("humidity map can be found");
    //println!("humidity: {}, {:?}", &humidity, &humidity_map);

    let location = humidity_map.convert_source_to_dest(humidity);
    //println!("location: {}", &location);

    //println!("seed: {seed}, location: {location}");
    //println!("");
    location
}

pub fn process(input: &str) -> u64 {
    let input = input.replace("\r\n", "\n");
    let (input, seeds) = parse_seeds(&input).expect("seeds can be found");

    let mut seed_maps = input.split("\n\n").collect::<Vec<&str>>().into_iter();

    let (_, seed_to_soil)            = parse_map(seed_maps.next().unwrap()).expect("seed_to_soil can be found");
    let (_, soil_to_fertilizer)      = parse_map(seed_maps.next().unwrap()).expect("soil_to_fertilizer can be found");
    let (_, fertilizer_to_water)     = parse_map(seed_maps.next().unwrap()).expect("fertilizer_to_water can be found");
    let (_, water_to_light)          = parse_map(seed_maps.next().unwrap()).expect("water_to_light can be found");
    let (_, light_to_temperature)    = parse_map(seed_maps.next().unwrap()).expect("light_to_temperature can be found");
    let (_, temperature_to_humidity) = parse_map(seed_maps.next().unwrap()).expect("temperature_to_humidity can be found");
    let (_, humidity_to_location)    = parse_map(seed_maps.next().unwrap()).expect("humidity_to_location can be found");

    let maps = vec![seed_to_soil, soil_to_fertilizer, fertilizer_to_water, water_to_light, light_to_temperature, temperature_to_humidity, humidity_to_location];

    seeds.iter().fold(u64::MAX, |mut lowest_location, seed| {
        let location = get_location_from_seed(seed, &maps);
        if location < lowest_location {
            lowest_location = location;
        }
        lowest_location
    })
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn example_input() {
        let input = "seeds: 79 14 55 13

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
56 93 4";
        let result = process(&input);
        assert_eq!(result, 46);
    }

    #[ignore]
    #[test]
    fn real_answer() {
        let input = std::env::current_dir().unwrap().display().to_string() + "/src/input.txt";
        let input = std::fs::read_to_string(input).expect("input to exist");
        let result = process(&input);
        assert_eq!(result, 0);
    }
}
