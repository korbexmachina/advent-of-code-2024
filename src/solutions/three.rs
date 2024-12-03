use regex;
use std::io::Read;

const INPUT_FILE: &str = "./input/day-3.txt";

fn read_input() -> Result<String, std::io::Error> {
    let mut infile: std::fs::File = std::fs::File::open(INPUT_FILE)?;
    let mut buf = String::new();
    infile.read_to_string(&mut buf)?;
    Ok(buf)
}

fn get_valid_statements(input: &String) -> Result<Vec<String>, std::io::Error> {
    let re = regex::Regex::new(r"(mul\(\d{1,3},\d{1,3}\))")
        .expect("Error: Failed to compile regular expression.");
    let mut valid_strings: Vec<String> = vec![];
    for (_, [value]) in re.captures_iter(input).map(|x| x.extract()) {
        valid_strings.push(value.to_owned());
    }
    Ok(valid_strings)
}

fn get_statements_as_ints(input: &String) -> Result<Vec<(usize, usize)>, std::io::Error> {
    let re = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")
        .expect("Error: Failed to compile regular expression.");
    let mut pairs: Vec<(usize, usize)> = vec![];
    for (_, [x, y]) in re.captures_iter(input).map(|n| n.extract()) {
        pairs.push((
            x.parse::<usize>().expect("Error: Bad number format."),
            y.parse::<usize>().expect("Error: Bad number format."),
        ));
    }
    Ok(pairs)
}

fn multiply_add(input: &Vec<(usize, usize)>) -> usize {
    input.iter().map(|(x, y)| x * y).sum()
}

#[test]
fn day_three_test() {
    let input = read_input().expect("Error: Unable to read input file.");
    let pairs = get_statements_as_ints(&input).expect("Error: Parser failed.");
    let solution = multiply_add(&pairs);
    println!("--- Day 3 ---\nFinal sum: {:#?}", solution);
}
