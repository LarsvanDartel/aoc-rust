use std::fmt::{Debug, Display};
use std::ops::{Index, IndexMut};

use winnow::ascii::line_ending;
use winnow::error::ParserError;
use winnow::stream::{Compare, Stream, StreamIsPartial};
use winnow::Parser;

use super::{list, many, Vec2};

type DisplayFn<T> = Box<dyn Fn(Vec2<isize>, &T) -> String>;

pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    data: Vec<T>,
    display_fn: Option<DisplayFn<T>>,
}

struct Coordinate(Vec2<usize>);

impl<T: Default> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: (0..width * height).map(|_| Default::default()).collect(),
            display_fn: None,
        }
    }
}

impl<T: Clone> Grid<T> {
    pub fn new_default(width: usize, height: usize, default: T) -> Self {
        Self {
            width,
            height,
            data: (0..width * height).map(|_| default.clone()).collect(),
            display_fn: None,
        }
    }
}

impl<T> Grid<T> {
    pub fn parse<I, E, P>(parser: P) -> impl Parser<I, Grid<T>, E>
    where
        I: StreamIsPartial + Stream + Compare<&'static str>,
        P: Parser<I, T, E>,
        E: ParserError<I>,
    {
        list(many(parser), line_ending).map(|data| {
            let height = data.len();
            let width = data.first().map_or(0, |row| row.len());
            for row in &data {
                assert_eq!(row.len(), width);
            }
            let data = data.into_iter().flatten().collect();
            Grid {
                width,
                height,
                data,
                display_fn: None,
            }
        })
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
            display_fn: None,
        }
    }

    pub fn map<U, F: FnMut(Vec2<isize>, &T) -> U>(&self, mut f: F) -> Grid<U> {
        Grid {
            width: self.width,
            height: self.height,
            data: self
                .coordinates()
                .map(|c| f(c, &self[c]))
                .collect::<Vec<U>>(),
            display_fn: None,
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

    pub fn with_display_fn<F: Fn(Vec2<isize>, &T) -> String + 'static>(self, f: F) -> Self {
        Self {
            display_fn: Some(Box::new(f)),
            ..self
        }
    }
}

impl<T, U> Grid<U>
where
    U: IntoIterator<Item = T>,
{
    pub fn flatten(self) -> Grid<T> {
        let data = self.data.into_iter().flatten().collect::<Vec<_>>();
        let width = data.len() / self.height;
        Grid {
            width,
            height: self.height,
            data,
            display_fn: None,
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
                if let Some(display_fn) = &self.display_fn {
                    write!(
                        f,
                        "{}",
                        display_fn(
                            Vec2::new(x as isize, y as isize),
                            &self[Coordinate(Vec2::new(x, y))]
                        )
                    )?;
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

impl<T: Clone> Clone for Grid<T> {
    fn clone(&self) -> Self {
        Self {
            width: self.width,
            height: self.height,
            data: self.data.clone(),
            display_fn: None,
        }
    }
}
