#![feature(array_chunks)]
#![feature(binary_heap_drain_sorted)]

use std::collections::BinaryHeap;

use regex::Regex;

#[timed::timed]
fn main() {
    let input = include_str!("../input.txt");
    let input = read_all_integers(input);

    let mut left_list = BinaryHeap::new();
    let mut right_list = BinaryHeap::new();

    for &[left, right] in input.array_chunks::<2>() {
        left_list.push(left);
        right_list.push(right);
    }

    let answer: u64 = left_list
        .drain_sorted()
        .zip(right_list.drain_sorted())
        .map(|(left, right)| left.abs_diff(right))
        .sum();

    dbg!(answer);
}

fn read_all_integers(text: &str) -> Vec<u64> {
    let iter = Regex::new(r"\d+").unwrap();
    let iter = iter.find_iter(text);
    iter.map(|x| x.as_str().parse::<u64>().unwrap()).collect()
}
