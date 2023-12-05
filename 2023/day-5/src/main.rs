use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{digit1, line_ending, multispace0, multispace1},
    combinator::{map, map_res, verify},
    error::{Error, ErrorKind},
    multi::many1,
    sequence::{pair, terminated},
    IResult,
};
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    let stdin = std::io::stdin();

    while let Ok(chars_read) = stdin.read_line(&mut buffer) {
        if chars_read == 0 {
            break;
        }
    }

    println!("{buffer}");

    let (_, almanac) = Almanac::parse(&buffer).unwrap();
    println!("almanac: {almanac:?}");
    let mapped_seeds = almanac.mapped_seeds();
    println!("\nmapped seeds: {:?}", mapped_seeds);

    let min_location = mapped_seeds.iter().map(|s| s.location).min().unwrap();
    println!("\nminimum location: {min_location}");

    Ok(())
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Item {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}
impl Item {
    pub fn parse(input: &str) -> IResult<&str, Item> {
        map_res(
            alt((
                tag("seed"),
                tag("soil"),
                tag("fertilizer"),
                tag("water"),
                tag("light"),
                tag("temperature"),
                tag("humidity"),
                tag("location"),
            )),
            |item: &str| match item {
                "seed" => Ok(Item::Seed),
                "soil" => Ok(Item::Soil),
                "fertilizer" => Ok(Item::Fertilizer),
                "water" => Ok(Item::Water),
                "light" => Ok(Item::Light),
                "temperature" => Ok(Item::Temperature),
                "humidity" => Ok(Item::Humidity),
                "location" => Ok(Item::Location),
                _ => Err(()),
            },
        )(input)
    }
}

#[derive(Clone, Debug)]
struct Almanac {
    seeds: Vec<usize>,
    seed_soil_map: Map,
    soil_fertilizer_map: Map,
    fertilizer_water_map: Map,
    water_light_map: Map,
    light_temperature_map: Map,
    temperature_humidity_map: Map,
    humidity_location_map: Map,
}
impl Almanac {
    pub fn mapped_seeds(&self) -> Vec<MappedSeed> {
        let mut seeds = vec![];

        for seed in &self.seeds {
            let soil = self.seed_soil_map.map(Item::Seed, *seed);
            let fertilizer = self.soil_fertilizer_map.map(Item::Soil, soil);
            let water = self.fertilizer_water_map.map(Item::Fertilizer, fertilizer);
            let light = self.water_light_map.map(Item::Water, water);
            let temperature = self.light_temperature_map.map(Item::Light, light);
            let humidity = self
                .temperature_humidity_map
                .map(Item::Temperature, temperature);
            let location = self.humidity_location_map.map(Item::Humidity, humidity);

            seeds.push(MappedSeed {
                seed: *seed,
                soil,
                fertilizer,
                water,
                light,
                temperature,
                humidity,
                location,
            });
        }

        seeds
    }
    pub fn parse(input: &str) -> IResult<&str, Almanac> {
        let mut seed_soil_map: Option<Map> = None;
        let mut soil_fertilizer_map: Option<Map> = None;
        let mut fertilizer_water_map: Option<Map> = None;
        let mut water_light_map: Option<Map> = None;
        let mut light_temperature_map: Option<Map> = None;
        let mut temperature_humidity_map: Option<Map> = None;
        let mut humidity_location_map: Option<Map> = None;

        let (input, seeds) = Almanac::parse_seeds(input)?;
        let (input, maps) = many1(Map::parse)(input)?;

        for mut map in maps.into_iter() {
            map.items.sort();

            if map.items == [Item::Seed, Item::Soil] {
                seed_soil_map = Some(map);
            } else if map.items == [Item::Soil, Item::Fertilizer] {
                soil_fertilizer_map = Some(map);
            } else if map.items == [Item::Fertilizer, Item::Water] {
                fertilizer_water_map = Some(map);
            } else if map.items == [Item::Water, Item::Light] {
                water_light_map = Some(map);
            } else if map.items == [Item::Light, Item::Temperature] {
                light_temperature_map = Some(map);
            } else if map.items == [Item::Temperature, Item::Humidity] {
                temperature_humidity_map = Some(map);
            } else if map.items == [Item::Humidity, Item::Location] {
                humidity_location_map = Some(map);
            }
        }

        let almanac = Almanac {
            seeds,
            seed_soil_map: seed_soil_map
                .ok_or(nom::Err::Failure(Error::new(input, ErrorKind::Verify)))?,
            soil_fertilizer_map: soil_fertilizer_map
                .ok_or(nom::Err::Failure(Error::new(input, ErrorKind::Verify)))?,
            fertilizer_water_map: fertilizer_water_map
                .ok_or(nom::Err::Failure(Error::new(input, ErrorKind::Verify)))?,
            water_light_map: water_light_map
                .ok_or(nom::Err::Failure(Error::new(input, ErrorKind::Verify)))?,
            light_temperature_map: light_temperature_map
                .ok_or(nom::Err::Failure(Error::new(input, ErrorKind::Verify)))?,
            temperature_humidity_map: temperature_humidity_map
                .ok_or(nom::Err::Failure(Error::new(input, ErrorKind::Verify)))?,
            humidity_location_map: humidity_location_map
                .ok_or(nom::Err::Failure(Error::new(input, ErrorKind::Verify)))?,
        };

        Ok((input, almanac))
    }
    fn parse_seeds(input: &str) -> IResult<&str, Vec<usize>> {
        let (input, _) = multispace0(input)?;
        let (input, _) = tag("seeds: ")(input)?;
        many1(map_res(terminated(digit1, multispace1), |s: &str| {
            s.parse::<usize>()
        }))(input)
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
struct MappedSeed {
    pub seed: usize,
    pub soil: usize,
    pub fertilizer: usize,
    pub water: usize,
    pub light: usize,
    pub temperature: usize,
    pub humidity: usize,
    pub location: usize,
}

#[derive(Clone, Debug)]
struct Map {
    pub items: [Item; 2],
    pub ranges: Vec<MapRange>,
}
impl Map {
    pub fn parse(input: &str) -> IResult<&str, Map> {
        let (input, _) = multispace0(input)?;
        let (input, item0) = Item::parse(input)?;
        let (input, _) = tag("-to-")(input)?;
        let (input, item1) = Item::parse(input)?;
        let (input, _) = tag(" map:")(input)?;
        let (input, _) = line_ending(input)?;
        let (input, ranges) = many1(terminated(MapRange::parse, line_ending))(input)?;
        let (input, _) = multispace0(input)?;

        Ok((
            input,
            Map {
                items: [item0, item1],
                ranges,
            },
        ))
    }
    pub fn can_map(&self, item: Item) -> bool {
        self.items.contains(&item)
    }
    pub fn map(&self, item: Item, index: usize) -> usize {
        if self.items[0] == item {
            for range in &self.ranges {
                if let Some(index) = range.map_forward(index) {
                    return index;
                }
            }
        } else if self.items[1] == item {
            for range in &self.ranges {
                if let Some(index) = range.map_back(index) {
                    return index;
                }
            }
        }

        index
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct MapRange {
    pub source_start: usize,
    pub destination_start: usize,
    pub length: usize,
}
impl MapRange {
    pub fn parse(input: &str) -> IResult<&str, MapRange> {
        let (input, destination_start) = map_res(digit1, |s: &str| s.parse::<usize>())(input)?;
        let (input, _) = multispace1(input)?;
        let (input, source_start) = map_res(digit1, |s: &str| s.parse::<usize>())(input)?;
        let (input, _) = multispace1(input)?;
        let (input, length) = map_res(digit1, |s: &str| s.parse::<usize>())(input)?;

        Ok((
            input,
            MapRange {
                source_start,
                destination_start,
                length,
            },
        ))
    }
    pub fn map_forward(&self, index: usize) -> Option<usize> {
        if self.source_start <= index && index < self.source_start + self.length {
            Some(self.destination_start + index - self.source_start)
        } else {
            None
        }
    }
    pub fn map_back(&self, index: usize) -> Option<usize> {
        if self.destination_start <= index && index < self.destination_start + self.length {
            Some(self.source_start + index - self.destination_start)
        } else {
            None
        }
    }
}
