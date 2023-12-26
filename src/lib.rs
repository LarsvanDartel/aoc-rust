mod error;

use clap::Parser;

pub use error::Error;
pub use error::Result;

pub type ParseResult<'a, T> = nom::IResult<&'a str, T>;

#[derive(Parser, Debug)]
pub struct Args {
    input_file: std::path::PathBuf,
}

impl Args {
    pub fn input(&self) -> Result<String> {
        std::fs::read_to_string(&self.input_file).map_err(|e| e.into())
    }
}

pub trait Problem<T1 = (), T2 = ()>
where
    T1: std::fmt::Debug,
    T2: std::fmt::Debug,
    Self: Sized,
{
    fn parse(input: &str) -> ParseResult<Self> {
        Self::parse_1(input)
    }
    fn parse_1(input: &str) -> ParseResult<Self> {
        Self::parse(input)
    }
    fn parse_2(input: &str) -> ParseResult<Self> {
        Self::parse(input)
    }
    fn part1(self) -> Result<T1>;
    fn part2(self) -> Result<T2>;
}

#[macro_export]
macro_rules! aoc_main {
    ($problem:ty) => {
        fn main() -> Result<()> {
            let input = aoc_main!(@input);

            let task1 = <$problem>::parse_1(&input);
            let task1 = aoc_main!(@finalize task1)?.1;

            println!("🎄 Running part 1...");

            let start = std::time::Instant::now();
            let result1 = task1.part1()?;
            let duration = start.elapsed();

            println!("🎄 Task 1: {:?}", result1);
            println!("🎄 Task 1 took: {:?}", duration);
            println!();

            let task2 = <$problem>::parse_2(&input);
            let task2 = aoc_main!(@finalize task2)?.1;

            println!("🎄 Running part 2...");
            let start = std::time::Instant::now();
            let result2 = task2.part2()?;
            let duration = start.elapsed();

            println!("🎄 Task 2: {:?}", result2);
            println!("🎄 Task 2 took: {:?}", duration);

            Ok(())
        }
    };
    (@input) => {
        <$crate::Args as ::clap::Parser>::parse().input()?
    };
    (@finalize $input:expr) => {
        ::nom::Finish::finish($input)
    }
}

#[macro_export]
macro_rules! assert_task {
    ($problem:ty, $task:expr, $input:expr, $expected:expr) => {{
        let input = $input;
        let task = <$problem>::parse(&input);
        let task = aoc_main!(@finalize task).unwrap().1;

        match $task {
            1 => assert_eq!(format!("{:?}", task.part1().unwrap()), format!("{:?}", $expected)),
            2 => assert_eq!(format!("{:?}", task.part2().unwrap()), format!("{:?}", $expected)),
            _ => panic!("Invalid task number"),
        }
    }};
}
