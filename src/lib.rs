pub mod common;
mod error;

use clap::Parser;

pub use error::AoCError;
pub use error::Result;
use winnow::PResult;

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
    fn parse(input: &mut &str) -> PResult<Self> {
        Self::parse_1(input)
    }
    fn parse_1(input: &mut &str) -> PResult<Self> {
        Self::parse(input)
    }
    fn parse_2(input: &mut &str) -> PResult<Self> {
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

            println!("🎄 Running part 1...");

            let mut input_1 = input.as_str();
            let start = std::time::Instant::now();
            let task1 = <$problem>::parse_1(&mut input_1)?;
            let duration = start.elapsed();

            println!("🎄 Task 1 parsed in: {:?}", duration);

            let start = std::time::Instant::now();
            let result1 = task1.part1()?;
            let duration = start.elapsed();

            println!("🎄 Task 1: {:?}", result1);
            println!("🎄 Task 1 took: {:?}", duration);
            println!();
            println!("🎄 Running part 2...");


            let mut input_2 = input.as_str();
            let start = std::time::Instant::now();
            let task2 = <$problem>::parse_2(&mut input_2)?;
            let duration = start.elapsed();

            println!("🎄 Task 2 parsed in: {:?}", duration);

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
}

#[macro_export]
macro_rules! assert_task {
    ($problem:ty, $task:expr, $input:expr, $expected:expr) => {{
        let mut input = $input.trim();

        let task = match $task {
            1 => <$problem>::parse_1(&mut input),
            2 => <$problem>::parse_2(&mut input),
            _ => panic!("Invalid task number"),
        }
        .unwrap();

        match $task {
            1 => assert_eq!(
                format!("{:?}", task.part1().unwrap()),
                format!("{:?}", $expected)
            ),
            2 => assert_eq!(
                format!("{:?}", task.part2().unwrap()),
                format!("{:?}", $expected)
            ),
            _ => panic!("Invalid task number"),
        }
    }};
}
