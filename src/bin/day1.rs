use std::collections::btree_set::BTreeSet;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::ops::{Add, Sub};

type Value = i64;
type Set = BTreeSet<Value>;

trait Integer: Add<Output = Self> + Sub<Output = Self> + Ord + Eq + Copy {}
impl<T: Add<Output = Self> + Sub<Output = Self> + Ord + Eq + Copy> Integer for T {}

// static mut ITERATIONS: u64 = 0;
static ITERATIONS: AtomicUsize = AtomicUsize::new(0);

trait SetIterator<I: Integer>: DoubleEndedIterator<Item = I> {

    fn find_n_sum(mut self, n: u8, target: I) -> Option<Vec<I>> where
        Self: Sized + Clone
    {
        match n {
            0..=1 => None,
            2     => {
                let mut head = self.next();
                let mut tail = self.next_back();
                while head.is_some() && tail.is_some() {
                    let _ = ITERATIONS.fetch_add(1, Ordering::Relaxed);
                    let h = head.unwrap();
                    let t = tail.unwrap();
                    let sum = h + t;
                    if sum == target {
                        return Some(vec![h, t]);
                    }
                    if sum < target {
                        head = self.next();
                    }
                    if sum > target {
                        tail = self.next_back();
                    }
                }
                None
            },
            _     => {
                let mut head = self.next();
                while head.is_some() {
                    let h = head.unwrap();
                    if let Some(mut found) = self.clone().find_n_sum(n - 1, target - h) {
                        found.push(h);
                        return Some(found);
                    }
                    head = self.next();
                }
                None
            }
        }
    }

}

impl<T, I: Integer> SetIterator<I> for T where T: DoubleEndedIterator<Item = I> {}

fn items<P: AsRef<std::path::Path>>(path: P) -> Set {
    let input = std::fs::read_to_string(path).unwrap();
    input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

pub fn main() {
    let mut args = std::env::args();
    let path = args.nth(1)
        .expect("Please provide path to the input file");
    let target: Value = args.next()
        .expect("Please provide target expense sum")
        .parse()
        .expect("Target sum must be an integer");
    let items = items(path);
    let result = items.iter().copied().find_n_sum(3, target);
    println!("{:?}", result);
    match result {
        Some(found) => println!("Product: {}", found.iter().product::<i64>()),
        None => {}
    }
    println!("Total iterations: {}", ITERATIONS.load(Ordering::Acquire));
}
