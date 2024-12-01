mod commands;
mod error;

use std::path::PathBuf;

use chrono::{Datelike, FixedOffset};
use clap::Parser;
pub(crate) use error::Result;

const CARGO_ROOT: &str = env!("CARGO_MANIFEST_DIR");

#[derive(Parser, Debug)]
enum Args {
    #[clap(name = "scaffold")]
    Scaffold {
        #[clap(short, long)]
        year: Option<u16>,

        #[clap(short, long)]
        day: Option<u8>,
    },

    #[clap(name = "solve")]
    Solve {
        #[clap(short, long)]
        year: Option<u16>,

        #[clap(short, long)]
        day: Option<u8>,

        #[clap(short, long)]
        path: Option<PathBuf>,

        #[clap(long)]
        submit: Option<u8>,
    },

    #[clap(name = "test")]
    Test {
        #[clap(short, long)]
        year: Option<u16>,
        #[clap(short, long)]
        day: Option<u8>,
    },
}

#[derive(Debug, Clone, Copy)]
struct AocDate {
    year: u16,
    day: u8,
}

impl AocDate {
    fn recent() -> Self {
        // aoc is based in UTC-5
        let tz = FixedOffset::west_opt(5 * 3600).unwrap();
        let now = chrono::Utc::now().with_timezone(&tz);

        let year = match now.month() {
            12 => now.year(),
            _ => now.year() - 1,
        } as u16;

        let day = match now.month() {
            12 => now.day().min(25),
            _ => 25,
        } as u8;

        Self { year, day }
    }

    fn input_path(&self) -> Result<PathBuf> {
        let mut path = PathBuf::from(CARGO_ROOT);
        path.push("input");
        self.push_path(&mut path)?;
        path.set_extension("txt");
        Ok(path)
    }

    fn bin_name(&self) -> String {
        format!("{:0>4}-{:0>2}", self.year, self.day)
    }

    fn bin_path(&self) -> Result<PathBuf> {
        let mut path = PathBuf::from(CARGO_ROOT);
        path.push("src");
        path.push("solutions");
        self.push_path(&mut path)?;
        path.set_extension("rs");
        Ok(path)
    }

    fn push_path(&self, path: &mut PathBuf) -> Result<()> {
        path.push(self.year.to_string());
        if !path.exists() {
            std::fs::create_dir_all(path.clone())?;
        }
        path.push(format!("day-{:0>2}", self.day));
        Ok(())
    }

    fn check_date(&self) -> Result<()> {
        let now = Self::recent();
        if self.year < 2015 || self.year > now.year {
            Err(*self)?
        }
        if self.day < 1 || self.day > 25 {
            Err(*self)?
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let mut date = AocDate::recent();

    let cleanup_year = |year| {
        if year < 100 {
            year + 2000
        } else {
            year
        }
    };

    match Args::parse() {
        Args::Scaffold { year, day } => {
            if let Some(year) = year {
                date.year = cleanup_year(year);
            }
            if let Some(day) = day {
                date.day = day;
            }
            date.check_date()?;
            commands::scaffold(date)?;
        }
        Args::Solve {
            year,
            day,
            path,
            submit,
        } => {
            if let Some(year) = year {
                date.year = cleanup_year(year);
            }
            if let Some(day) = day {
                date.day = day;
            }
            date.check_date()?;
            commands::solve(date, path, submit)?;
        }
        Args::Test { year, day } => {
            if let Some(year) = year {
                date.year = cleanup_year(year);
            }
            if let Some(day) = day {
                date.day = day;
            }
            date.check_date()?;
            commands::test(date)?;
        }
    }

    Ok(())
}
