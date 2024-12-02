use std::{fs::File, io::BufRead, path::Path};

const INPUT_FILE: &str = "./input/day-2.txt";

fn parse_input() -> Result<Vec<Vec<usize>>, std::io::Error> {
    let lines = read_lines(INPUT_FILE)?;
    let mut big_vector: Vec<Vec<usize>> = vec![];
    let mut small_vector: Vec<usize>;
    for line in lines.flatten() {
        // Pull the numbers out into a vector
        small_vector = line
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        big_vector.push(small_vector); // Update the main vector
    }
    Ok(big_vector)
}

fn read_lines<P>(filename: P) -> std::io::Result<std::io::Lines<std::io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(std::io::BufReader::new(file).lines())
}

fn validate_safety(mut report: Vec<usize>, threshold: usize) -> bool {
    let mut threshold_count = 0;
    if !report.is_sorted() {
        // println!("{:?} -> {}", report, report.is_sorted());
        report.reverse();
        // println!("{:?} -> {}", report, report.is_sorted());
        if !report.is_sorted() {
            threshold_count += 1;
            if threshold_count > threshold {
                return false;
            }
            for index in 0..report.len() {
                let mut new_report: Vec<usize> = report
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i != index)
                    .map(|(_, x)| *x)
                    .collect();
                if new_report.is_sorted() {
                    return validate_safety(new_report, threshold - 1);
                }
                new_report.reverse();
                if new_report.is_sorted() {
                    return validate_safety(new_report, threshold - 1);
                }
            }
        }
    }
    let mut lock: bool = true;
    let mut prev: usize = 0;
    for index in 0..report.len() {
        if lock {
            lock = false;
            prev = report[index];
        } else {
            let diff = report[index].abs_diff(prev);
            // println!("{}", diff);
            if diff > 3 || diff == 0 {
                // println!("{:?} -> invalid", &report);
                threshold_count += 1;
                if threshold_count > threshold {
                    return false;
                }
                let new_report: Vec<usize> = report
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i != index)
                    .map(|(_, x)| *x)
                    .collect();
                return validate_safety(new_report, threshold - 1);
            }
            prev = report[index];
        }
    }
    // println!("{:?} -> valid", &report);
    true
}

fn get_safe_reports(reports: Vec<Vec<usize>>, threshold: usize) -> usize {
    let mut count = 0;
    for report in reports {
        if validate_safety(report, threshold) {
            count += 1;
        }
    }
    count
}

#[test]
fn day_two_test() {
    let reports = parse_input().expect("ERROR: Failed to parse input.");
    let safe_reports = get_safe_reports(reports.clone(), 0);
    let dampener_safe_reports = get_safe_reports(reports.clone(), 1);
    println!(
        "--- DAY 2 ---\nNumber of safe reports: {}\nNumber of safe reports with Dampener: {}",
        safe_reports, dampener_safe_reports
    );
}
