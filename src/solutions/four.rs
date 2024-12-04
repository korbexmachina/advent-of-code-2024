use regex;
use std::io::Read;

const INPUT_FILE: &str = "./input/day-4.txt";
// const INPUT_FILE: &str = "./input/day-4-test.txt";

#[derive(Debug, Clone)]
struct State {
    matches: usize,
    x_mas: usize,
    matrix: Vec<Vec<char>>,
}

fn initialize_state() -> Result<State, std::io::Error> {
    let mut infile = std::fs::File::open(INPUT_FILE)?;
    let mut buf: String = String::new();
    let mut state: State = State {
        matches: 0,
        x_mas: 0,
        matrix: vec![vec![]],
    };
    infile.read_to_string(&mut buf)?;
    let lines: Vec<String> = buf.split_whitespace().map(|x| x.to_owned()).collect();
    lines
        .iter()
        .for_each(|x| state.matrix.push(x.chars().collect()));
    Ok(state)
}

fn forward_search<'a>(
    state: &'a mut State,
    re: &regex::Regex,
    re2: &regex::Regex,
) -> Result<&'a mut State, std::io::Error> {
    state.matrix.iter().for_each(|line| {
        state.matches += re.find_iter(&line.iter().collect::<String>()).count();
        state.matches += re2.find_iter(&line.iter().collect::<String>()).count();
    });
    Ok(state)
}

fn diagonal_search<'a>(
    state: &'a mut State,
    re: &regex::Regex,
    re2: &regex::Regex,
) -> Result<&'a mut State, std::io::Error> {
    let mut up_right: Vec<Vec<char>> = vec![vec![]; 2000];
    let mut down_right: Vec<Vec<char>> = vec![vec![]; 2000];
    state.matrix.iter().enumerate().for_each(|(index, line)| {
        line.iter()
            .enumerate()
            .for_each(|(i, c)| up_right[index + i].push(*c));
    });
    // println!("up_right: {:#?}", up_right); // TEST:
    up_right.iter().for_each(|line| {
        state.matches += re.find_iter(&line.iter().collect::<String>()).count();
        state.matches += re2.find_iter(&line.iter().collect::<String>()).count();
    });
    // state.matrix.iter().enumerate().for_each(|(index, line)| {
    //     line.iter()
    //         .enumerate()
    //         .for_each(|(i, c)| down_right[index.abs_diff(i)].push(*c));
    // });
    state
        .matrix
        .iter()
        .rev()
        .enumerate()
        .for_each(|(index, line)| {
            line.iter()
                .enumerate()
                .for_each(|(i, c)| down_right[index + i].push(*c));
        });
    // println!("down_right: {:#?}", down_right); // TEST:
    down_right.iter().for_each(|line| {
        state.matches += re.find_iter(&line.iter().collect::<String>()).count();
        state.matches += re2.find_iter(&line.iter().collect::<String>()).count();
    });
    Ok(state)
}

fn vertical_search<'a>(
    state: &'a mut State,
    re: &regex::Regex,
    re2: &regex::Regex,
) -> Result<&'a mut State, std::io::Error> {
    let mut columns: Vec<Vec<char>> = vec![vec![]; state.matrix[1].len()];
    // Rotate the matrix 90 degrees
    state.matrix.iter().for_each(|line| {
        line.iter()
            .enumerate()
            .for_each(|(i, c)| columns[i].push(*c));
    });
    // Now we can just scan the lines, since they are the columns of the original matrix
    columns.iter().for_each(|line| {
        state.matches += re.find_iter(&line.iter().collect::<String>()).count();
        state.matches += re2.find_iter(&line.iter().collect::<String>()).count();
    });
    Ok(state)
}

#[test]
fn day_three_test() {
    let mut state = initialize_state().unwrap();
    let re = regex::Regex::new(r"XMAS").expect("Error: Failed to compile regular expression.");
    let re2 = regex::Regex::new(r"SAMX").expect("Error: Failed to compile regular expression.");
    let state = forward_search(&mut state, &re, &re2).unwrap();
    let state = vertical_search(state, &re, &re2).unwrap();
    let state = diagonal_search(state, &re, &re2).unwrap();
    println!("--- DAY 4 ---\nMatches: {}", state.matches);
}
