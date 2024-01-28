use aoc_rust::*;
use nom::{
    bytes::complete::tag,
    character::complete::{i32 as number, line_ending},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    const fn cost(&self) -> i32 {
        match self {
            Self::MagicMissile => 53,
            Self::Drain => 73,
            Self::Shield => 113,
            Self::Poison => 173,
            Self::Recharge => 229,
        }
    }

    const fn all() -> [Self; 5] {
        [
            Self::MagicMissile,
            Self::Drain,
            Self::Shield,
            Self::Poison,
            Self::Recharge,
        ]
    }
}

#[derive(Debug, Clone)]
struct Player {
    hp: i32,
    mana: i32,
    armor: i32,
    effects: Vec<(Spell, i32)>,
}

#[derive(Debug, Clone)]
struct Boss {
    hp: i32,
    damage: i32,
}

impl Player {
    fn new(hp: i32, mana: i32) -> Self {
        Self {
            hp,
            mana,
            armor: 0,
            effects: Vec::new(),
        }
    }

    fn apply_effects(&mut self, boss: &mut Boss) {
        for (spell, turns) in self.effects.iter_mut() {
            match spell {
                Spell::Shield => {
                    if *turns == 1 {
                        self.armor = 0;
                    }
                }
                Spell::Poison => {
                    boss.hp -= 3;
                }
                Spell::Recharge => {
                    self.mana += 101;
                }
                _ => {}
            }
            *turns -= 1;
        }
        self.effects.retain(|(_, turns)| *turns > 0);
    }

    fn cast(&mut self, spell: Spell, boss: &mut Boss) -> bool {
        if self.effects.iter().any(|(s, _)| s == &spell) {
            return false;
        }
        if self.mana < spell.cost() {
            return false;
        }
        self.mana -= spell.cost();
        match spell {
            Spell::MagicMissile => {
                boss.hp -= 4;
            }
            Spell::Drain => {
                boss.hp -= 2;
                self.hp += 2;
            }
            Spell::Shield => {
                self.armor = 7;
                self.effects.push((Spell::Shield, 6));
            }
            Spell::Poison => {
                self.effects.push((Spell::Poison, 6));
            }
            Spell::Recharge => {
                self.effects.push((Spell::Recharge, 5));
            }
        }
        true
    }
}

#[derive(Debug, Clone)]
struct Day22 {
    player: Player,
    boss: Boss,
}

impl Day22 {
    fn min_mana(&self, hard: bool) -> i32 {
        let mut min_mana = i32::MAX;
        let mut queue = Vec::new();
        queue.push((self.player.clone(), self.boss.clone(), 0));

        while let Some((mut player, mut boss, mana)) = queue.pop() {
            if mana >= min_mana {
                continue;
            }
            player.apply_effects(&mut boss);
            if boss.hp <= 0 {
                min_mana = min_mana.min(mana);
                continue;
            }
            if hard {
                player.hp -= 1;
                if player.hp <= 0 {
                    continue;
                }
            }
            for spell in Spell::all().iter() {
                let mut player = player.clone();
                let mut boss = boss.clone();
                if player.cast(*spell, &mut boss) {
                    player.apply_effects(&mut boss);
                    if boss.hp <= 0 {
                        min_mana = min_mana.min(mana + spell.cost());
                        continue;
                    }
                    player.hp -= (boss.damage - player.armor).max(1);
                    if player.hp <= 0 {
                        continue;
                    }
                    queue.push((player, boss, mana + spell.cost()));
                }
            }
        }

        min_mana
    }
}

impl Problem<i32, i32> for Day22 {
    fn parse(input: &str) -> ParseResult<Self> {
        let (input, _) = tag("Hit Points: ")(input)?;
        let (input, hp) = number(input)?;
        let (input, _) = line_ending(input)?;
        let (input, _) = tag("Damage: ")(input)?;
        let (input, damage) = number(input)?;
        let (input, _) = line_ending(input)?;
        Ok((
            input,
            Self {
                player: Player::new(50, 500),
                boss: Boss { hp, damage },
            },
        ))
    }

    fn part1(self) -> Result<i32> {
        Ok(self.min_mana(false))
    }

    fn part2(self) -> Result<i32> {
        Ok(self.min_mana(true))
    }
}

aoc_main!(Day22);
