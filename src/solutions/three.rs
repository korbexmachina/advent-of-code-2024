use std::{char, io::Read, iter::Peekable};

const INPUT_FILE: &str = "./input/day-3.txt";

#[derive(Debug, Clone)]
pub enum Token {
    Mul,
    Number(u32),
    Paren,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub children: Vec<Node>,
    pub entry: Token,
}

impl Node {
    pub fn new() -> Node {
        Node {
            children: Vec::new(),
            entry: Token::Mul,
        }
    }
}

#[derive(Debug, Clone)]
pub enum LexItem {
    Op(String),
    Paren(char),
    Num(u32),
}

fn read_input() -> Result<String, std::io::Error> {
    let mut infile: std::fs::File = std::fs::File::open(INPUT_FILE)?;
    let mut buf = String::new();
    infile.read_to_string(&mut buf)?;
    Ok(buf)
}

fn get_number<T: Iterator<Item = char>>(c: char, iter: &mut Peekable<T>) -> u32 {
    let mut number = c
        .to_string()
        .parse::<u32>()
        .expect("The caller should have passed a digit.");
    while let Some(Ok(num)) = iter.peek().map(|x| x.to_string().parse::<u32>()) {
        number = number * 10 + num; // Constructing the number
        iter.next();
    }
    number
}

fn get_op<T: Iterator<Item = char>>(c: char, iter: &mut Peekable<T>) -> Option<String> {
    let mut str = c.to_string();
    while let Some(&c) = iter.peek() {
        match c {
            'a'..='z' => {
                str = format!("{}{}", str, c);
                iter.next();
            }
            _ => break,
        }
    }
    match str.as_str() {
        "mul" => return Some(str),
        _ => return None,
    }
}

fn lex(input: &String) -> Result<Vec<LexItem>, String> {
    let mut result = Vec::new();

    let mut buf = input.chars().peekable();
    while let Some(&c) = buf.peek() {
        match c {
            '0'..='9' => {
                buf.next();
                let n = get_number(c, &mut buf);
                result.push(LexItem::Num(n));
            }
            'm' => {
                buf.next();
                if let Some(x) = get_op(c, &mut buf) {
                    result.push(LexItem::Op(x));
                }
            }
            '(' | ')' => {
                result.push(LexItem::Paren(c));
                buf.next();
            }
            _ => {
                buf.next();
            }
        }
    }
    println!("{:#?}", result);
    Ok(result)
}

fn parse(input: String) -> Result<Vec<Node>, String> {
    let tokens = lex(&input)?;

    parse_expr(&tokens, 0).and_then(|(x, i)| {
        if i == tokens.len() {
            Ok(n)
        } else {
            Err("Parse Error!".to_owned())
        }
    })
}

fn parse_expr(input: &Vec<LexItem>, index: usize) -> Result<(Node, usize), String> {
    // TODO: Implement
}

fn parse_mul(input: &Vec<LexItem>, index: usize) -> Result<(Node, usize), String> {
    // TODO: Implement
}

#[test]
fn day_three_test() {
    let input = read_input().expect("Error: Unable to read input file.");
    let lex_out = parse(&input).expect("Error: Parser failed.");
    println!("--- Day 3 ---");
}
