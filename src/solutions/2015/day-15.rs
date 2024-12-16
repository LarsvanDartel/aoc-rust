use aoc_rust::*;
use common::*;

struct Day15 {
    ingredients: Vec<Ingredient>,
}

#[derive(Debug, Clone)]
struct Ingredient {
    #[allow(unused)]
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Ingredient {
    fn parse(input: &mut &str) -> PResult<Self> {
        let name = alpha1(input)?;
        let _ = ": capacity ".parse_next(input)?;
        let capacity = dec_int(input)?;
        let _ = ", durability ".parse_next(input)?;
        let durability = dec_int(input)?;
        let _ = ", flavor ".parse_next(input)?;
        let flavor = dec_int(input)?;
        let _ = ", texture ".parse_next(input)?;
        let texture = dec_int(input)?;
        let _ = ", calories ".parse_next(input)?;
        let calories = dec_int(input)?;
        Ok(Ingredient {
            name: name.to_string(),
            capacity,
            durability,
            flavor,
            texture,
            calories,
        })
    }
}

impl Problem<i32, i32> for Day15 {
    fn parse(input: &mut &str) -> PResult<Self> {
        separated(0.., Ingredient::parse, line_ending)
            .map(|ingredients| Self { ingredients })
            .parse_next(input)
    }

    fn part1(self) -> Result<i32> {
        let mut max_score = 0;

        for i in 0..=100 {
            for j in 0..=(100 - i) {
                for k in 0..=(100 - i - j) {
                    let l = 100 - i - j - k;
                    let capacity = self.ingredients[0].capacity * i
                        + self.ingredients[1].capacity * j
                        + self.ingredients[2].capacity * k
                        + self.ingredients[3].capacity * l;
                    let durability = self.ingredients[0].durability * i
                        + self.ingredients[1].durability * j
                        + self.ingredients[2].durability * k
                        + self.ingredients[3].durability * l;
                    let flavor = self.ingredients[0].flavor * i
                        + self.ingredients[1].flavor * j
                        + self.ingredients[2].flavor * k
                        + self.ingredients[3].flavor * l;
                    let texture = self.ingredients[0].texture * i
                        + self.ingredients[1].texture * j
                        + self.ingredients[2].texture * k
                        + self.ingredients[3].texture * l;
                    if capacity < 0 || durability < 0 || flavor < 0 || texture < 0 {
                        continue;
                    }
                    let score = capacity * durability * flavor * texture;
                    max_score = max_score.max(score);
                }
            }
        }

        Ok(max_score)
    }

    fn part2(self) -> Result<i32> {
        let mut max_score = 0;

        for i in 0..=100 {
            for j in 0..=(100 - i) {
                for k in 0..=(100 - i - j) {
                    let l = 100 - i - j - k;
                    let capacity = self.ingredients[0].capacity * i
                        + self.ingredients[1].capacity * j
                        + self.ingredients[2].capacity * k
                        + self.ingredients[3].capacity * l;
                    let durability = self.ingredients[0].durability * i
                        + self.ingredients[1].durability * j
                        + self.ingredients[2].durability * k
                        + self.ingredients[3].durability * l;
                    let flavor = self.ingredients[0].flavor * i
                        + self.ingredients[1].flavor * j
                        + self.ingredients[2].flavor * k
                        + self.ingredients[3].flavor * l;
                    let texture = self.ingredients[0].texture * i
                        + self.ingredients[1].texture * j
                        + self.ingredients[2].texture * k
                        + self.ingredients[3].texture * l;
                    let calories = self.ingredients[0].calories * i
                        + self.ingredients[1].calories * j
                        + self.ingredients[2].calories * k
                        + self.ingredients[3].calories * l;
                    if calories != 500 {
                        continue;
                    }
                    if capacity < 0 || durability < 0 || flavor < 0 || texture < 0 {
                        continue;
                    }
                    let score = capacity * durability * flavor * texture;
                    max_score = max_score.max(score);
                }
            }
        }

        Ok(max_score)
    }
}

aoc_main!(Day15);
