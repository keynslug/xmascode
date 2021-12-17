use std::{
    convert::{TryFrom, TryInto},
    fmt::{Display, Formatter},
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

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Open => write!(f, "."),
            Tile::Tree => write!(f, "#")
        }
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

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                self.get((x, y)).fmt(f)?
            }
            write!(f, "\n")?
        }
        Ok(())
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

pub fn main() {

    let mut args = std::env::args();
    let path = args.nth(1).expect("Please provide path to the input file");
    let input = std::fs::read_to_string(path).unwrap();

    let grid: Grid = str::parse(&input).unwrap();
    println!("{}", grid);

    let routes = vec![
        grid.route((1, 1)),
        grid.route((3, 1)),
        grid.route((5, 1)),
        grid.route((7, 1)),
        grid.route((1, 2))
    ];

    let hits = routes.into_iter().map(|route| {
        route.filter(|&tile| tile == Tile::Tree).count()
    });

    let product = hits.fold(1, |acc, hs| {
        println!("Hits: {}", hs);
        acc * hs
    });

    println!("Product: {}", product);

}
