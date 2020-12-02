use rust_advent_of_code::_2020::_01::Report;

use std::fs;

#[test]
fn _01 () {
    let filename = "inputs/_2020/_01/1.txt";

    let input = fs::read_to_string(filename)
                            .unwrap()
                            .split("\n")
                            .filter(|x| !x.is_empty())
                            .map(|x| str::parse::<i32>(x).unwrap())
                            .collect::<Vec<i32>>();

    let report = Report::from_vector(&input);

    println!("{}", report.find_and_multiply());
}