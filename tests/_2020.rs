use rust_advent_of_code::_2020::_01::Report;
use rust_advent_of_code::_2020::_02::PasswordEntry;
use rust_advent_of_code::_2020::_03::Grid;

use std::fs;

#[test]
fn _01_part_one () {
    let filename = "inputs/_2020/_01/1.txt";

    let input = fs::read_to_string(filename)
                            .unwrap()
                            .split("\n")
                            .filter(|x| !x.is_empty())
                            .map(|x| str::parse::<i32>(x).unwrap())
                            .collect::<Vec<i32>>();

    let report = Report::from_vector(input);

    println!("{}", report.part_one());
}

#[test]
fn _01_part_two () {
    let filename = "inputs/_2020/_01/1.txt";

    let input = fs::read_to_string(filename)
                            .unwrap()
                            .split("\n")
                            .filter(|x| !x.is_empty())
                            .map(|x| str::parse::<i32>(x).unwrap())
                            .collect::<Vec<i32>>();

    let report = Report::from_vector(input);

    println!("{}", report.part_two());
}

#[test]
fn _2020_02_part_one () {
    let filename = "inputs/_2020/_02/1.txt";
    let result = fs::read_to_string(filename)
                          .unwrap()
                          .split("\n")
                          .filter(|x| !x.is_empty())
                          .map(|input| PasswordEntry::from_str(input).is_valid())
                          .map(|x| if x { 1 } else { 0 })
                          .sum::<i32>();

    println!("{}", result);
}

#[test]
fn _2020_02_part_two () {
    let filename = "inputs/_2020/_02/1.txt";
    let result = fs::read_to_string(filename)
                          .unwrap()
                          .split("\n")
                          .filter(|x| !x.is_empty())
                          .map(|input| PasswordEntry::from_str(input).is_valid2())
                          .map(|x| if x { 1 } else { 0 })
                          .sum::<i32>();

    println!("{}", result);
}

#[test]
fn _2020_03_part_one () {
    let filename = "inputs/_2020/_03/1.txt";
    let input = fs::read_to_string(filename).unwrap();
    let grid = Grid::from_str(input.trim());
    let result = grid.count_trees();

    println!("{}", result);
}
