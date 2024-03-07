/*
    Appellation: default <bench>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![feature(test)]

extern crate test;

use std::mem::replace;
use test::Bencher;

// bench: find the `BENCH_SIZE` first terms of the fibonacci sequence
static BENCH_SIZE: usize = 20;

// recursive fibonacci
fn fibonacci(n: usize) -> u32 {
    if n < 2 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

// iterative fibonacci
pub struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Fibonacci {
    pub fn new(curr: u32, next: u32) -> Fibonacci {
        Fibonacci { curr, next }
    }

    pub fn seq() -> Fibonacci {
        Fibonacci::new(1, 1)
    }
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;
        let new_curr = replace(&mut self.next, new_next);

        Some(replace(&mut self.curr, new_curr))
    }
}

// function to benchmark must be annotated with `#[bench]`
#[bench]
fn recursive_fibonacci(b: &mut Bencher) {
    // exact code to benchmark must be passed as a closure to the iter
    // method of Bencher
    b.iter(|| (0..BENCH_SIZE).map(fibonacci).collect::<Vec<u32>>())
}

#[bench]
fn iterative_fibonacci(b: &mut Bencher) {
    b.iter(|| Fibonacci::seq().take(BENCH_SIZE).collect::<Vec<u32>>())
}
