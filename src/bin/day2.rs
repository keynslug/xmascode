extern crate peg;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

#[derive(Debug)]
pub struct Policy {
    range: RangeInclusive<usize>,
    c: char
}

peg::parser! {
    grammar password_policy_parser() for str {
        rule __()
            = [' ' | '\t']+
        rule _()
            = [' ' | '\t']*
        rule char() -> char
            = c:$([_]) { c.chars().next().unwrap() }
        rule integer() -> usize
            = n:$(['1'..='9']['0'..='9']+ / ['0'..='9']) {? n.parse().or(Err("usize")) }
        rule policy() -> Policy
            = from:integer() "-" to:integer() __ c:char() { Policy {range: from..=to, c} }
        pub rule root() -> (Policy, &'input str)
            = p:policy() _ ":" _ pwd:$([^ ' ' | '\t']+) { (p, pwd) }
    }
}

pub fn main() {
    let mut args = std::env::args();
    let path = args.nth(1)
        .expect("Please provide path to the input file");
    let file = File::open(path).unwrap();
    let mut valid = 0;
    let lines = BufReader::new(file).lines();
    for line in lines {
        let line = line.unwrap();
        let (policy, password) = password_policy_parser::root(&line).unwrap();
        // let occurences = password.chars().filter(|&c| c == policy.c).count();
        // if policy.range.contains(&occurences) {
        //     valid += 1;
        // }
        let occurences = password.chars()
            .enumerate()
            .filter(|&(i, c)| {
                (i == policy.range.start() - 1 || i == policy.range.end() - 1) &&
                    c == policy.c
            })
            .count();
        if occurences == 1 {
            valid += 1;
        }
    }
    println!("Valid: {}", valid);
}
