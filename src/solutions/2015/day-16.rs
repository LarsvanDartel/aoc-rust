use aoc_rust::*;
use common::*;

struct Day16 {
    mfcsam: Sue,
    sues: Vec<Sue>,
}

#[derive(Debug)]
struct Sue {
    number: u32,
    children: Option<u32>,
    cats: Option<u32>,
    samoyeds: Option<u32>,
    pomeranians: Option<u32>,
    akitas: Option<u32>,
    vizslas: Option<u32>,
    goldfish: Option<u32>,
    trees: Option<u32>,
    cars: Option<u32>,
    perfumes: Option<u32>,
}

impl Sue {
    fn parse(input: &mut &str) -> PResult<Self> {
        let _ = "Sue ".parse_next(input)?;
        let number = dec_uint(input)?;
        let _ = ": ".parse_next(input)?;

        let mut sue = Sue {
            number,
            children: None,
            cats: None,
            samoyeds: None,
            pomeranians: None,
            akitas: None,
            vizslas: None,
            goldfish: None,
            trees: None,
            cars: None,
            perfumes: None,
        };

        let properties: Vec<(&str, u32)> =
            separated(0.., separated_pair(alpha1, ": ", dec_uint), ", ").parse_next(input)?;

        for (property, value) in properties {
            match property {
                "children" => sue.children = Some(value),
                "cats" => sue.cats = Some(value),
                "samoyeds" => sue.samoyeds = Some(value),
                "pomeranians" => sue.pomeranians = Some(value),
                "akitas" => sue.akitas = Some(value),
                "vizslas" => sue.vizslas = Some(value),
                "goldfish" => sue.goldfish = Some(value),
                "trees" => sue.trees = Some(value),
                "cars" => sue.cars = Some(value),
                "perfumes" => sue.perfumes = Some(value),
                _ => panic!("Unknown property: {}", property),
            }
        }

        Ok(sue)
    }

    fn matches(&self, mfcam: &Sue, retroencabulator: bool) -> bool {
        self.children.map_or(true, |v| v == mfcam.children.unwrap())
            && self.cats.map_or(true, |v| {
                if retroencabulator {
                    v > mfcam.cats.unwrap()
                } else {
                    v == mfcam.cats.unwrap()
                }
            })
            && self.samoyeds.map_or(true, |v| v == mfcam.samoyeds.unwrap())
            && self.pomeranians.map_or(true, |v| {
                if retroencabulator {
                    v < mfcam.pomeranians.unwrap()
                } else {
                    v == mfcam.pomeranians.unwrap()
                }
            })
            && self.akitas.map_or(true, |v| v == mfcam.akitas.unwrap())
            && self.vizslas.map_or(true, |v| v == mfcam.vizslas.unwrap())
            && self.goldfish.map_or(true, |v| {
                if retroencabulator {
                    v < mfcam.goldfish.unwrap()
                } else {
                    v == mfcam.goldfish.unwrap()
                }
            })
            && self.trees.map_or(true, |v| {
                if retroencabulator {
                    v > mfcam.trees.unwrap()
                } else {
                    v == mfcam.trees.unwrap()
                }
            })
            && self.cars.map_or(true, |v| v == mfcam.cars.unwrap())
            && self.perfumes.map_or(true, |v| v == mfcam.perfumes.unwrap())
    }
}

impl Problem<u32, u32> for Day16 {
    fn parse(input: &mut &str) -> PResult<Self> {
        separated(0.., Sue::parse, line_ending)
            .map(|sues| {
                let mfcsam = Sue {
                    number: 0,
                    children: Some(3),
                    cats: Some(7),
                    samoyeds: Some(2),
                    pomeranians: Some(3),
                    akitas: Some(0),
                    vizslas: Some(0),
                    goldfish: Some(5),
                    trees: Some(3),
                    cars: Some(2),
                    perfumes: Some(1),
                };
                Self { mfcsam, sues }
            })
            .parse_next(input)
    }

    fn part1(self) -> Result<u32> {
        let sue = self
            .sues
            .iter()
            .find(|sue| sue.matches(&self.mfcsam, false))
            .unwrap();
        Ok(sue.number)
    }

    fn part2(self) -> Result<u32> {
        let sue = self
            .sues
            .iter()
            .find(|sue| sue.matches(&self.mfcsam, true))
            .unwrap();
        Ok(sue.number)
    }
}

aoc_main!(Day16);
