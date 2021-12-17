use criterion::{Criterion, black_box, criterion_group, criterion_main};

use std::{
    convert::{TryFrom, TryInto},
    str::FromStr
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Open,
    Tree
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Open
    }
}

#[derive(Debug)]
struct ParseError(String);

impl ParseError {
    pub fn new(s: impl Into<String>) -> Self {
        ParseError(s.into())
    }
}

impl TryFrom<char> for Tile {
    type Error = ParseError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Tile::Open),
            '#' => Ok(Tile::Tree),
            _ => Err(ParseError::new(format!("Invalid tile: {}", c)))
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize
}

impl From<(usize, usize)> for Coord {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct Grid {
    size: Coord,
    tiles: Vec<Tile>
}

impl Grid {

    pub fn new(size: impl Into<Coord>) -> Grid {
        let size = size.into();
        Grid {
            size,
            tiles: vec![Tile::default(); size.x * size.y]
        }
    }

    pub fn get(&self, coord: impl Into<Coord>) -> Tile {
        self.tiles[self.index(coord.into())]
    }

    pub fn set(&mut self, coord: impl Into<Coord>, tile: Tile) {
        let index = self.index(coord.into());
        self.tiles[index] = tile
    }

    pub fn route(&self, step: impl Into<Coord>) -> impl Iterator<Item = Tile> + '_ {
        Route::new(self.size, step.into()).map(move |coord| self.get(coord))
    }

    fn index(&self, coord: Coord) -> usize {
        assert!(coord.y < self.size.y);
        coord.y * self.size.x + coord.x % self.size.x
    }

}

impl FromStr for Grid {

    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let height = s.lines().count();
        let mut width = None;
        for line in s.lines() {
            let w = line.len();
            match width {
                Some(w0) => {
                    if w0 != w {
                        return Err(ParseError::new(format!("Irregular input: rows has lengths: {} vs {}", w0, w)))
                    }
                }
                None => {
                    width = Some(w)
                }
            }
        }

        let width = width.unwrap_or(0);
        let mut grid = Grid::new((width, height));
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let tile: Tile = c.try_into()?;
                grid.set((x, y), tile)
            }
        }

        Ok(grid)
    }

}

#[derive(Debug)]
struct Route {
    size: Coord,
    coord: Coord,
    step: Coord
}

impl Route {
    pub fn new(size: Coord, step: Coord) -> Self {
        Self { size, coord: (0, 0).into(), step }
    }
}

impl Iterator for Route {

    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.coord.y < self.size.y {
            let next = Some(self.coord);
            self.coord.x += self.step.x;
            self.coord.y += self.step.y;
            next
        }
        else {
            None
        }
    }

}

pub fn benchmark(c: &mut Criterion) {
    let grid: Grid = std::fs::read_to_string("input/day3").unwrap().parse().unwrap();
    c.bench_function("day3 dumb", |b| {
        b.iter(|| {
            black_box(grid.route((3, 1)).filter(|&t| t == Tile::Tree).count())
        });
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
