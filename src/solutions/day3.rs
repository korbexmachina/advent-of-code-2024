use regex;
use std::io::Read;

const INPUT_FILE: &str = "./input/day-3.txt";

#[derive(Debug, Clone)]
// Tracks the program state
struct State {
    enabled: bool,
    program: Vec<Node>,
}

#[derive(Debug, Clone)]
// Represents syntax node
enum Node {
    Mul(usize, usize),
    Do,
    Dont,
}

fn read_input() -> Result<String, std::io::Error> {
    let mut infile: std::fs::File = std::fs::File::open(INPUT_FILE)?;
    let mut buf = String::new();
    infile.read_to_string(&mut buf)?;
    Ok(buf)
}

fn get_valid_statements(input: &String) -> Result<State, std::io::Error> {
    let re = regex::Regex::new(r"(mul\(\d{1,3},\d{1,3}\)|don't\(\)|do\(\))")
        .expect("Error: Failed to compile regular expression.");
    let args = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")
        .expect("Error: Failed to compile regular expression.");
    let mut state = State {
        enabled: true,
        program: vec![],
    };
    for (_, [value]) in re.captures_iter(input).map(|x| x.extract()) {
        match value {
            "do()" => state.program.push(Node::Do),
            "don't()" => state.program.push(Node::Dont),
            _ => {
                if let Some((_, [x, y])) = args.captures(value).map(|n| n.extract()) {
                    state.program.push(Node::Mul(
                        x.parse::<usize>().expect("Error: Bad number format."),
                        y.parse::<usize>().expect("Error: Bad number format."),
                    ));
                }
            }
        }
    }
    Ok(state)
}

fn multiply_add_state(state: &mut State) -> usize {
    let mut total: usize = 0;
    for node in &state.program {
        match node {
            Node::Do => {
                state.enabled = true;
            }
            Node::Dont => {
                state.enabled = false;
            }
            Node::Mul(x, y) => {
                if state.enabled {
                    total += x * y;
                }
            }
        }
    }
    total
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
    let mut state =
        get_valid_statements(&input).expect("Error: Failed to initialize program state.");
    let solution2 = multiply_add_state(&mut state);
    println!(
        "--- Day 3 ---\nFinal sum: {:#?}\nPart 2: {:#?}",
        solution, solution2
    );
}
