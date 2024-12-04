use nom::character::complete::line_ending;
use nom::error::ParseError;
use nom::{Compare, IResult, InputIter, InputLength, Parser, Slice};
use std::fmt::{Debug, Display};
use std::ops::{Index, IndexMut, Range, RangeFrom, RangeTo};

use super::Vec2;

type GridDisplay<T> = Box<dyn Fn(&T) -> char>;

pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    data: Vec<T>,
    display: Option<GridDisplay<T>>,
}

struct Coordinate(Vec2<usize>);

impl<T: Default> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: (0..width * height).map(|_| Default::default()).collect(),
            display: None,
        }
    }
}

impl<T> Grid<T> {
    pub fn parse<I, E, F>(mut f: F) -> impl FnMut(I) -> IResult<I, Grid<T>, E>
    where
        I: Clone
            + InputLength
            + InputIter
            + Slice<Range<usize>>
            + Slice<RangeFrom<usize>>
            + Slice<RangeTo<usize>>,
        I: Compare<&'static str>,
        F: Parser<I, T, E>,
        E: ParseError<I>,
    {
        move |mut input| {
            let mut data = Vec::new();
            let mut width = 0;
            let mut height = 0;

            while let Ok((i, value)) = f.parse(input.clone()) {
                data.push(value);
                input = i;
                width = data.len();
            }

            if !data.is_empty() {
                height += 1;
            }

            while let Ok((i, _)) = line_ending::<I, E>(input.clone()) {
                input = i;

                let mut row_width = 0;
                while let Ok((i, value)) = f.parse(input.clone()) {
                    row_width += 1;
                    data.push(value);
                    input = i;
                }

                if row_width == 0 {
                    break;
                }

                if row_width != width {
                    return Err(nom::Err::Error(E::from_error_kind(
                        input,
                        nom::error::ErrorKind::SeparatedList,
                    )));
                }

                height += 1;
            }

            assert_eq!(data.len(), width * height);

            Ok((
                input,
                Grid {
                    width,
                    height,
                    data,
                    display: None,
                },
            ))
        }
    }

    pub fn set_display<F: Fn(&T) -> char + 'static>(&mut self, f: F) {
        self.display = Some(Box::new(f));
    }

    fn coord<C: Into<Vec2<isize>>>(&self, c: C) -> Option<Coordinate> {
        let c: Vec2<isize> = c.into();
        if c.x < 0 || c.y < 0 || c.x >= self.width as isize || c.y >= self.height as isize {
            None
        } else {
            Some(Coordinate(c.map(|x| x as usize)))
        }
    }

    pub fn coordinates(&self) -> impl Iterator<Item = Vec2<isize>> + '_ {
        (0..self.height)
            .flat_map(move |y| (0..self.width).map(move |x| Vec2::new(x as isize, y as isize)))
    }

    pub fn get(&self, pos: Vec2<isize>) -> Option<&T> {
        Some(&self[self.coord(pos)?])
    }

    pub fn get_mut(&mut self, pos: Vec2<isize>) -> Option<&mut T> {
        let c = self.coord(pos)?;
        Some(&mut self[c])
    }

    pub fn set(&mut self, pos: Vec2<isize>, value: T) {
        if let Some(c) = self.coord(pos) {
            self[c] = value;
        }
    }
}

impl<T> Index<Coordinate> for Grid<T> {
    type Output = T;

    fn index(&self, index: Coordinate) -> &Self::Output {
        &self.data[index.0.y * self.width + index.0.x]
    }
}

impl<T> IndexMut<Coordinate> for Grid<T> {
    fn index_mut(&mut self, index: Coordinate) -> &mut Self::Output {
        &mut self.data[index.0.y * self.width + index.0.x]
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(display) = &self.display {
                    write!(f, "{}", display(&self[Coordinate(Vec2::new(x, y))]))?;
                } else {
                    write!(f, "{}", self[Coordinate(Vec2::new(x, y))])?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T: Debug> Debug for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{:?}", self[Coordinate(Vec2::new(x, y))])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
