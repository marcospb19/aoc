#![feature(array_chunks)]

use itertools::Itertools;
use regex::Regex;

#[timed::timed]
fn main() {
    let input = include_str!("../input.txt");
    let input = read_all_integers(input);

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for &[left, right] in input.array_chunks::<2>() {
        left_list.push(left);
        right_list.push(right);
    }

    let frequency = right_list.into_iter().counts();

    let answer = left_list
        .into_iter()
        .map(|id| id * frequency.get(&id).unwrap_or(&0))
        .sum::<usize>();

    dbg!(answer);
}

fn read_all_integers(text: &str) -> Vec<usize> {
    let iter = Regex::new(r"\d+").unwrap();
    let iter = iter.find_iter(text);
    iter.map(|x| x.as_str().parse::<usize>().unwrap()).collect()
}
