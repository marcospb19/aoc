#![feature(array_windows)]

use regex::Regex;

#[timed::timed]
fn main() {
    let input = include_str!("../input.txt");

    let mut answer = 0;

    for line in input.lines() {
        let nums = read_all_integers(line);

        let nums_whole = nums.array_windows::<2>().copied();
        let mut nums_with_skip =
            (0..nums.len()).map(|skip_index| VecSkipIter::new(&nums, skip_index));

        if is_safe(nums_whole) || nums_with_skip.any(is_safe) {
            answer += 1;
        }
    }

    dbg!(answer);
}

fn is_safe<I>(iter: I) -> bool
where
    I: IntoIterator<Item = [usize; 2]>,
{
    let mut increasing = true;
    let mut decreasing = true;
    let mut diff_limit = true;

    for [a, b] in iter {
        increasing &= a < b;
        decreasing &= a > b;
        diff_limit &= matches!(a.abs_diff(b), 1..=3);
    }

    (increasing || decreasing) && diff_limit
}

struct VecSkipIter<'a> {
    array: &'a [usize],
    index_to_skip: usize,
    position: usize,
}

impl<'a> VecSkipIter<'a> {
    pub fn new(array: &'a [usize], index_to_skip: usize) -> Self {
        Self {
            array,
            index_to_skip,
            position: 0,
        }
    }
}

impl Iterator for VecSkipIter<'_> {
    type Item = [usize; 2];

    fn next(&mut self) -> Option<Self::Item> {
        if self.position == self.index_to_skip {
            self.position += 1;
        }

        let &first = self.array.get(self.position)?;
        self.position += 1;

        if self.position == self.index_to_skip {
            self.position += 1;
        }

        let &second = self.array.get(self.position)?;
        Some([first, second])
    }
}

fn read_all_integers(text: &str) -> Vec<usize> {
    let iter = Regex::new(r"\d+").unwrap();
    let iter = iter.find_iter(text);
    iter.map(|x| x.as_str().parse::<usize>().unwrap()).collect()
}
