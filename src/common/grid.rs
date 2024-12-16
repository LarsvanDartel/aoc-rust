use std::fmt::{Debug, Display};
use std::ops::{Index, IndexMut};

use winnow::ascii::line_ending;
use winnow::error::{ErrMode, ParserError};
use winnow::stream::{Compare, Stream, StreamIsPartial};
use winnow::{PResult, Parser};

use super::Vec2;

#[derive(Clone)]
pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    data: Vec<T>,
}

struct Coordinate(Vec2<usize>);

impl<T: Default> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: (0..width * height).map(|_| Default::default()).collect(),
        }
    }
}

impl<T> Grid<T> {
    pub fn parse<I, E, P>(parser: P) -> GridParser<P, I, T, E>
    where
        I: StreamIsPartial + Stream + Compare<&'static str>,
        P: Parser<I, T, E>,
        E: ParserError<I>,
    {
        GridParser {
            parser,
            i: Default::default(),
            o: Default::default(),
            e: Default::default(),
        }
    }
}

impl<T> Grid<T> {
    pub fn contains<C: Into<Vec2<isize>>>(&self, c: C) -> bool {
        let c: Vec2<isize> = c.into();
        c.x >= 0 && c.y >= 0 && c.x < self.width as isize && c.y < self.height as isize
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

    pub fn get<C: Into<Vec2<isize>>>(&self, pos: C) -> Option<&T> {
        Some(&self[self.coord(pos)?])
    }

    pub fn get_mut<C: Into<Vec2<isize>>>(&mut self, pos: C) -> Option<&mut T> {
        let c = self.coord(pos)?;
        Some(&mut self[c])
    }

    pub fn set<C: Into<Vec2<isize>>>(&mut self, pos: C, value: T) {
        if let Some(c) = self.coord(pos) {
            self[c] = value;
        }
    }

    pub fn flat_map<U, I, F: FnMut(T) -> I>(self, f: F) -> Grid<U>
    where
        I: IntoIterator<Item = U>,
    {
        let data = self.data.into_iter().flat_map(f).collect::<Vec<U>>();
        let width = data.len() / self.height;
        Grid {
            width,
            height: self.height,
            data,
        }
    }

    pub fn map<U, F: FnMut(T) -> U>(self, f: F) -> Grid<U> {
        Grid {
            width: self.width,
            height: self.height,
            data: self.data.into_iter().map(f).collect(),
        }
    }

    pub fn find(&self, value: T) -> Option<Vec2<isize>>
    where
        T: PartialEq,
    {
        self.coordinates().find(|&c| self[c] == value)
    }

    pub fn get_row(&self, row: usize) -> Option<&[T]> {
        if row < self.height {
            Some(&self.data[row * self.width..(row + 1) * self.width])
        } else {
            None
        }
    }

    pub fn get_column(&self, column: usize) -> Option<Vec<&T>> {
        if column < self.width {
            Some(
                (0..self.height)
                    .map(|y| &self.data[y * self.width + column])
                    .collect(),
            )
        } else {
            None
        }
    }
}

impl<T> Grid<Vec<T>> {
    pub fn flatten(self) -> Grid<T> {
        Grid {
            width: self.width,
            height: self.height,
            data: self.data.into_iter().flatten().collect(),
        }
    }
}

impl<T> Index<Vec2<isize>> for Grid<T> {
    type Output = T;

    fn index(&self, index: Vec2<isize>) -> &Self::Output {
        if let Some(c) = self.coord(index) {
            &self[c]
        } else {
            panic!("Index out of bounds")
        }
    }
}

impl<T> IndexMut<Vec2<isize>> for Grid<T> {
    fn index_mut(&mut self, index: Vec2<isize>) -> &mut Self::Output {
        if let Some(c) = self.coord(index) {
            &mut self[c]
        } else {
            panic!("Index out of bounds")
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
                write!(f, "{}", self[Coordinate(Vec2::new(x, y))])?;
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

pub struct GridParser<P, I, O, E>
where
    I: StreamIsPartial + Stream + Compare<&'static str>,
    P: Parser<I, O, E>,
    E: ParserError<I>,
{
    parser: P,
    i: core::marker::PhantomData<I>,
    o: core::marker::PhantomData<O>,
    e: core::marker::PhantomData<E>,
}

impl<P, I, O, E> Parser<I, Grid<O>, E> for GridParser<P, I, O, E>
where
    I: StreamIsPartial + Stream + Compare<&'static str>,
    P: Parser<I, O, E>,
    E: ParserError<I>,
{
    fn parse_next(&mut self, input: &mut I) -> PResult<Grid<O>, E> {
        let mut data = Vec::new();
        let mut height = 0;
        let mut width = 0;
        loop {
            let mut w = 0;
            loop {
                let start = input.checkpoint();
                let len = input.eof_offset();
                match self.parser.parse_next(input) {
                    Err(ErrMode::Backtrack(_)) => {
                        input.reset(&start);
                        break;
                    },
                    Err(e) => return Err(e),
                    Ok(o) => {
                        if input.eof_offset() == len {
                            return Err(ErrMode::assert(input, "Parsers must always consume"));
                        }
                        data.push(o);
                        w += 1;
                    },
                }
            }
            if width == 0 {
                width = w;
            }
            if w != width {
                if w == 0 {
                    return Ok(Grid {
                        width,
                        height,
                        data,
                    });
                }
                return Err(ErrMode::assert(input, "All rows must have the same width"));
            }
            height += 1;

            let start = input.checkpoint();
            let len = input.eof_offset();
            match line_ending::<I, E>.parse_next(input) {
                Err(ErrMode::Backtrack(_)) => {
                    input.reset(&start);
                    break;
                },
                Err(e) => return Err(e),
                Ok(_) => {
                    if input.eof_offset() == len {
                        return Err(ErrMode::assert(input, "Parsers must always consume"));
                    }
                },
            }
        }

        Ok(Grid {
            width,
            height,
            data,
        })
    }
}
