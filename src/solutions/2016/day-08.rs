use aoc_rust::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, u32 as parse_u32},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    Parser,
};

struct Day08 {
    operations: Box<[Operation]>,
}

struct Screen {
    width: usize,
    height: usize,
    pixels: Vec<bool>,
}

impl Screen {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![false; width * height],
        }
    }

    fn rect(&mut self, a: usize, b: usize) {
        for y in 0..b {
            for x in 0..a {
                self.pixels[y * self.width + x] = true;
            }
        }
    }

    fn rotate_row(&mut self, row: usize, by: u32) {
        let row = row * self.width;
        let mut new_row = vec![false; self.width];
        for x in 0..self.width {
            new_row[(x + by as usize) % self.width] = self.pixels[row + x];
        }
        self.pixels.splice(row..row + self.width, new_row);
    }

    fn rotate_column(&mut self, column: usize, by: u32) {
        let mut new_column = vec![false; self.height];
        for y in 0..self.height {
            new_column[(y + by as usize) % self.height] = self.pixels[y * self.width + column];
        }
        for (y, &pixel) in new_column.iter().enumerate() {
            self.pixels[y * self.width + column] = pixel;
        }
    }

    fn count_lit(&self) -> usize {
        self.pixels.iter().filter(|&&x| x).count()
    }

    fn execute(&mut self, operations: &[Operation]) {
        for operation in operations {
            match operation {
                Operation::Rect(a, b) => self.rect(*a, *b),
                Operation::RotateRow { row, by } => self.rotate_row(*row, *by),
                Operation::RotateColumn { column, by } => self.rotate_column(*column, *by),
            }
        }
    }
}

impl std::fmt::Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(
                    f,
                    "{}",
                    if self.pixels[y * self.width + x] {
                        '#'
                    } else {
                        '.'
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

enum Operation {
    Rect(usize, usize),
    RotateRow { row: usize, by: u32 },
    RotateColumn { column: usize, by: u32 },
}

impl Operation {
    fn parse(input: &str) -> ParseResult<Self> {
        fn parse_usize(input: &str) -> ParseResult<usize> {
            parse_u32.map(|x| x as usize).parse(input)
        }

        alt((
            preceded(
                tag("rect "),
                separated_pair(parse_usize, tag("x"), parse_usize)
                    .map(|(a, b)| Operation::Rect(a, b)),
            ),
            preceded(
                tag("rotate row y="),
                separated_pair(parse_usize, tag(" by "), parse_u32)
                    .map(|(row, by)| Operation::RotateRow { row, by }),
            ),
            preceded(
                tag("rotate column x="),
                separated_pair(parse_usize, tag(" by "), parse_u32)
                    .map(|(column, by)| Operation::RotateColumn { column, by }),
            ),
        ))
        .parse(input)
    }
}

impl Problem<usize, ()> for Day08 {
    fn parse(input: &str) -> ParseResult<Self> {
        separated_list1(line_ending, Operation::parse)
            .map(|operations| Day08 {
                operations: operations.into_boxed_slice(),
            })
            .parse(input)
    }

    fn part1(self) -> Result<usize> {
        let mut screen = Screen::new(50, 6);
        screen.execute(&self.operations);
        Ok(screen.count_lit())
    }

    fn part2(self) -> Result<()> {
        let mut screen = Screen::new(50, 6);
        screen.execute(&self.operations);
        println!("{}", screen);
        Ok(())
    }
}

aoc_main!(Day08);
