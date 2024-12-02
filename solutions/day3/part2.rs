use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");

    let mut answer = 0;
    let mut is_enabled = true;

    for token in read_all_tokens(input) {
        match token {
            Token::Mul(a, b) => {
                if is_enabled {
                    answer += a * b;
                }
            }
            Token::Do => is_enabled = true,
            Token::Dont => is_enabled = false,
        }
    }

    dbg!(answer);
}

enum Token {
    Mul(usize, usize),
    Do,
    Dont,
}

fn read_all_tokens(text: &str) -> Vec<Token> {
    let iter = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let iter = iter.captures_iter(text);
    iter.map(|capture| {
        let whole = capture.get(0).unwrap().as_str();

        if whole == "do()" {
            Token::Do
        } else if whole == "don't()" {
            Token::Dont
        } else {
            let left = capture.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let right = capture.get(2).unwrap().as_str().parse::<usize>().unwrap();
            Token::Mul(left, right)
        }
    })
    .collect()
}
