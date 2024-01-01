use aoc_rust::*;
use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, u32 as parse_u32},
};

const WEAPONS: [Item; 5] = [
    Item::new(8, 4, 0),
    Item::new(10, 5, 0),
    Item::new(25, 6, 0),
    Item::new(40, 7, 0),
    Item::new(74, 8, 0),
];

const ARMOR: [Item; 6] = [
    Item::new(0, 0, 0),
    Item::new(13, 0, 1),
    Item::new(31, 0, 2),
    Item::new(53, 0, 3),
    Item::new(75, 0, 4),
    Item::new(102, 0, 5),
];

const RINGS: [Item; 7] = [
    Item::new(0, 0, 0),
    Item::new(25, 1, 0),
    Item::new(50, 2, 0),
    Item::new(100, 3, 0),
    Item::new(20, 0, 1),
    Item::new(40, 0, 2),
    Item::new(80, 0, 3),
];

struct Day21 {
    boss: Character,
}

struct Character {
    hp: u32,
    damage: u32,
    armor: u32,
}

#[derive(Debug, Clone, Copy)]
struct Item {
    cost: u32,
    damage: u32,
    armor: u32,
}

impl Item {
    const fn new(cost: u32, damage: u32, armor: u32) -> Self {
        Self {
            cost,
            damage,
            armor,
        }
    }
}

impl std::ops::Add for Item {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            cost: self.cost + rhs.cost,
            damage: self.damage + rhs.damage,
            armor: self.armor + rhs.armor,
        }
    }
}

impl Character {
    fn beats(&self, other: &Self) -> bool {
        let self_turns =
            (other.hp as f32 / (self.damage as f32 - other.armor as f32).max(1.0)).ceil();
        let other_turns =
            (self.hp as f32 / (other.damage as f32 - self.armor as f32).max(1.0)).ceil();

        self_turns <= other_turns
    }
}

impl Problem<u32, u32> for Day21 {
    fn parse(input: &str) -> ParseResult<Self> {
        let (input, _) = tag("Hit Points: ")(input)?;
        let (input, hp) = parse_u32(input)?;
        let (input, _) = line_ending(input)?;
        let (input, _) = tag("Damage: ")(input)?;
        let (input, damage) = parse_u32(input)?;
        let (input, _) = line_ending(input)?;
        let (input, _) = tag("Armor: ")(input)?;
        let (input, armor) = parse_u32(input)?;
        let (input, _) = line_ending(input)?;

        Ok((
            input,
            Self {
                boss: Character { hp, damage, armor },
            },
        ))
    }

    fn part1(self) -> Result<u32> {
        let mut min_cost = u32::MAX;
        for weapon in WEAPONS.iter() {
            for armor in ARMOR.iter() {
                for (i, ring1) in RINGS.iter().enumerate() {
                    for (j, ring2) in RINGS.iter().enumerate() {
                        if i == j {
                            continue;
                        }

                        let total = *weapon + *armor + *ring1 + *ring2;
                        let player = Character {
                            hp: 100,
                            damage: total.damage,
                            armor: total.armor,
                        };

                        if player.beats(&self.boss) {
                            min_cost = min_cost.min(total.cost);
                        }
                    }
                }
            }
        }

        Ok(min_cost)
    }

    fn part2(self) -> Result<u32> {
        let mut max_cost = 0;
        for weapon in WEAPONS.iter() {
            for armor in ARMOR.iter() {
                for (i, ring1) in RINGS.iter().enumerate() {
                    for (j, ring2) in RINGS.iter().enumerate() {
                        if i == j {
                            continue;
                        }

                        let total = *weapon + *armor + *ring1 + *ring2;
                        let player = Character {
                            hp: 100,
                            damage: total.damage,
                            armor: total.armor,
                        };

                        if !player.beats(&self.boss) {
                            max_cost = max_cost.max(total.cost);
                        }
                    }
                }
            }
        }

        Ok(max_cost)
    }
}

aoc_main!(Day21);
