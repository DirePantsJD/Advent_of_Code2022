mod day1_2022;

use std::{
    env,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
use day1_2022::{most_calories, top_three_calories_sum};

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args[1].clone();
    let part = args[2].clone();
    let path = "input/".to_owned() + &(day)[..] + "_input.txt";
    let lines: Vec<String> = lines_from_file(path);

    match day.as_str() {
        "day1" => {
                   if part == "1" {println!("{}",most_calories(lines.clone()))};
                   if part == "2" {println!("{}",top_three_calories_sum(lines.clone()))}
                  },
        _ => (),
    }

}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}