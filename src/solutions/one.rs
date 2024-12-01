use std::{collections::hash_map::HashMap, fmt::Error, io::Read};

const INPUT_FILE: &str = "./input/day-1.txt";

fn read_input(path: &str) -> Result<(Vec<usize>, Vec<usize>), std::io::Error> {
    let mut infile = std::fs::File::open(path)?;
    let mut buffer = String::new();
    let _ = infile.read_to_string(&mut buffer);
    let all: Vec<&str> = buffer.split_whitespace().collect();
    // println!("{:?}", all);
    let all_ints: Vec<usize> = all
        .iter()
        .map(|x| x.to_owned().parse::<usize>().unwrap())
        .collect();
    // println!("{:?}", all_ints);
    let even: Vec<usize> = all_ints
        .iter()
        .enumerate()
        .filter(|&(i, _)| i % 2 == 0)
        .map(|(_, x)| *x)
        .collect();
    let odd: Vec<usize> = all_ints
        .iter()
        .enumerate()
        .filter(|&(i, _)| i % 2 != 0)
        .map(|(_, x)| *x)
        .collect();
    // println!("{:?}", &even);
    // println!("{:?}", &odd);
    Ok((even, odd))
}

fn get_distance(list1: Vec<usize>, list2: Vec<usize>) -> Result<usize, Error> {
    let mut l1 = list1;
    let mut l2 = list2;
    l1.sort();
    l2.sort();

    let distances: Vec<usize> = l1
        .iter()
        .enumerate()
        .map(|(y, x)| {
            let y = l2.get(y).unwrap();
            if x > y {
                x - y
            } else {
                y - x
            }
        })
        .collect();

    Ok(distances.iter().sum())
}

fn get_similarity(list1: Vec<usize>, list2: Vec<usize>) -> Result<usize, Error> {
    let freq: HashMap<usize, usize> = list2
        .iter()
        .map(|x| (*x, list2.iter().filter(|y| *y == x).count()))
        .collect();
    let mut distance = 0;
    for i in &list1 {
        distance += match freq.get(&i) {
            Some(n) => i * n,
            None => 0,
        }
    }
    Ok(distance)
}

#[test]
fn day_one_test() {
    let (list1, list2) = read_input(INPUT_FILE).unwrap();
    println!(
        "Distance: {}",
        get_distance(list1.clone(), list2.clone()).unwrap()
    );
    println!("Similarity: {}", get_similarity(list1, list2).unwrap());
}
