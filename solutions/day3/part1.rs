use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");

    let answer: usize = read_all_tokens(input).into_iter().map(|(a, b)| a * b).sum();
    dbg!(answer);
}

fn read_all_tokens(text: &str) -> Vec<(usize, usize)> {
    let iter = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let iter = iter.captures_iter(text);
    iter.map(|capture| {
        let left = capture.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let right = capture.get(2).unwrap().as_str().parse::<usize>().unwrap();
        (left, right)
    })
    .collect()
}
