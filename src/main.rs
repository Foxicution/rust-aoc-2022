// The only thing I don't like is we can't see what we import explicitly

use advent_of_code_2022_rust::{
    day1::Day1Solution,
    day2::{day2, day2_part2},
    day3::{day3, day3_part2},
    day4::{day4, day4_part2},
};

use std::fs::read_to_string;

fn main() {
    println!(
        "Day 1 solution: {:?}",
        read_to_string("./input/1").unwrap().day1()
    );

    let day2_in = read_to_string("./input/2").unwrap();
    println!(
        "Day 2 solution: {:?}, part 2: {:?}",
        day2(day2_in.as_str()),
        day2_part2(day2_in.as_str())
    );

    let day3_in = read_to_string("./input/3").unwrap();

    println!(
        "Day 3 solution: {:?}, part 2: {:?}",
        day3(day3_in.as_str()),
        day3_part2(day3_in.as_str())
    );

    let day4_in = read_to_string("./input/4").unwrap();

    println!(
        "Day 4 solution: {:?}, part 2 {}",
        day4(day4_in.as_str()),
        day4_part2(day4_in.as_str())
    );

    let input = "\n\
        [D]    \n\
    [N] [C]    \n\
    [Z] [M] [P]\n\
     1   2   3 \n\
    \n\
    move 1 from 2 to 1\n\
    move 3 from 1 to 3\n\
    move 2 from 2 to 1\n\
    move 1 from 1 to 2";

    println!("{}", input)
}
