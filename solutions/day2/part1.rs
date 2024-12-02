#![feature(array_windows)]

use regex::Regex;

#[timed::timed]
fn main() {
    let input = include_str!("../input.txt");

    let mut answer = 0;

    for line in input.lines() {
        let nums = read_all_integers(line.trim());
        if is_safe(nums) {
            answer += 1;
        }
    }

    dbg!(answer);
}

fn is_safe(candidate: Vec<usize>) -> bool {
    let increasing = candidate.array_windows::<2>().all(|[a, b]| a < b);
    let decreasing = candidate.array_windows::<2>().all(|[a, b]| a > b);

    let diff_limit = candidate
        .array_windows::<2>()
        .all(|&[a, b]| matches!(a.abs_diff(b), 1..=3));

    (increasing || decreasing) && diff_limit
}

fn read_all_integers(text: &str) -> Vec<usize> {
    let iter = Regex::new(r"\d+").unwrap();
    let iter = iter.find_iter(text);
    iter.map(|x| x.as_str().parse::<usize>().unwrap()).collect()
}
