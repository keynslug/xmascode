use criterion::{Criterion, black_box, criterion_group, criterion_main};
use bitset_core::BitSet;

type Lane = [u64; 4];
type BitIdx = usize;

const BITS_PER_LANE: usize = std::mem::size_of::<Lane>() * 8;

#[derive(Debug)]
struct Grid {
    v: Vec<Lane>,
    stride: BitIdx,
    cursor: BitIdx
}

struct GridRow<'a> {
    slice: &'a mut [Lane],
    offset: BitIdx,
    stride: BitIdx
}

impl<'a> GridRow<'a> {

    pub fn stride(&self) -> BitIdx {
        self.stride
    }

    pub fn bit_set(&mut self, bit: usize) -> &mut Self {
        self.slice.bit_set(self.offset + bit);
        self
    }

}

impl Grid {

    pub fn new(stride: usize) -> Self {
        Self { v: vec![Lane::default()], stride, cursor: 0 }
    }

    pub fn each(&self, from: usize, left: usize, down: usize) -> Each {
        Each { idx: 0, cursor: from, left, down, stride: self.stride }
    }

    pub fn len(&self) -> usize {
        self.v.len()
    }

    pub fn next_row(&mut self) -> GridRow {
        let cursor = self.cursor;
        self.cursor += self.stride;
        self.v.resize(1 + (self.cursor + self.stride - 1) / BITS_PER_LANE, Lane::default());
        GridRow {
            slice: self.v.as_mut_slice(),
            offset: cursor,
            stride: self.stride
        }
    }

}

#[derive(Debug)]
struct Each {
    idx: usize,
    cursor: usize,
    left: usize,
    down: usize,
    stride: usize
}

impl Iterator for Each {
    type Item = Lane;
    fn next(&mut self) -> Option<Self::Item> {
        let mut next = Lane::default();
        while self.idx < BITS_PER_LANE {
            next.bit_set(self.idx);
            self.idx += self.down * self.stride + self.left; // Stroll down and left.
            self.cursor += self.left; // Remember how far to the left we are.
            if self.cursor >= self.stride { // If we're past the stride already...
                self.cursor -= self.stride; // ...wrap back...
                self.idx -= self.stride; // ...and correct current position by going back `stride` bits.
            }
        }
        self.idx -= BITS_PER_LANE;
        Some(next)
    }
}

#[derive(Debug)]
struct ParseError(String);

impl ParseError {
    pub fn new(s: impl Into<String>) -> Self {
        ParseError(s.into())
    }
}

impl std::str::FromStr for Grid {

    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        fn parse_line(line: &str, mut slice: GridRow) -> Result<(), ParseError> {
            let length = line.len();
            let stride = slice.stride();
            if length != stride {
                return Err(ParseError::new(format!(
                    "Irregular input: line is {} bytes long, stride is {}",
                    length,
                    stride
                )))
            }
            for (idx, c) in (0..stride).zip(line.chars()) {
                match c {
                    '.' => {},
                    '#' => { slice.bit_set(idx); },
                    _ => {
                        return Err(ParseError::new(format!("Invalid char in input: {}", c)))
                    }
                }
            }
            Ok(())
        }

        let mut lines = s.lines();
        if let Some(first) = lines.next() {
            let stride = first.len();
            let mut grid = Grid::new(stride);
            parse_line(first, grid.next_row())?;
            for line in lines {
                parse_line(line, grid.next_row())?;
            }
            Ok(grid)
        }
        else {
            Ok(Grid::new(0))
        }

    }

}

// fn count_hits(grid: &Grid, each: Each) -> usize {
//     let mut path: Vec<Lane> = each.take(grid.len()).collect();
//     let lanes = grid.v.as_slice();
//     path.as_mut_slice().bit_and(lanes).bit_count()
// }

pub fn benchmark(c: &mut Criterion) {
    let grid: Grid = std::fs::read_to_string("input/day3").unwrap().parse().unwrap();
    let path: Vec<Lane> = grid.each(0, 3, 1).take(grid.len()).collect();
    c.bench_function("day3 bitset", |b| {
        b.iter(|| {
            let lanes = grid.v.as_slice();
            black_box(path.clone().as_mut_slice().bit_and(lanes).bit_count())
        })
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
