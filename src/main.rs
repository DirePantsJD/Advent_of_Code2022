mod day1_2022;
mod day2_2022;
mod day3_2022;
mod day4_2022;
mod day5_2022;
mod day6_2022;
mod day7_2022;
mod day8_2022;

use std::{
    env,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
use day1_2022::*;
use day2_2022::*;
use day3_2022::*;
use day4_2022::*;
use day5_2022::*;
use day6_2022::*;
use day7_2022::*;
use day8_2022::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args[1].clone();
    let part = args[2].clone();
    let path = "input/".to_owned() + &(day)[..] + "_input.txt";
    let lines: Vec<String> = lines_from_file(path.clone());

    match day.as_str() {
        "day1" => {
                   if part == "1" {println!("Result: {}",most_calories(lines.clone()))};
                   if part == "2" {println!("Result: {}",top_three_calories_sum(lines.clone()))}
                  },
        "day2" => {
                   if part == "1" {println!("Result: {}",calculate_score1(lines.clone()))}
                   if part == "2" {println!("Result: {}",calculate_score2(lines.clone()))}
                  },
        "day3" => {
                   if part == "1" {println!("Result: {}",sum_common_item_priority(lines.clone()))}
                   if part == "2" {println!("Result: {}",sum_badge_priority(lines.clone()))}
                  },
        "day4" => {
                   if part == "1" {println!("Result: {}",count_completly_redundant_ranges(lines.clone()))}
                   if part == "2" {println!("Result: {}",count_partialy_redundant_ranges(lines.clone()))}
                  },
        "day5" => {
                   if part == "1" {println!("Result: {}",top_of_stacks(lines.clone()))}
                   if part == "2" {println!("Result: {}",v2_top_of_stacks(lines.clone()))}
                  },
        "day6" => {
                   if part == "1" {println!("Result: {}",start_of_packet_offset(lines.clone()))}
                   if part == "2" {println!("Result: {}",start_of_message_offset(lines.clone()))}
                  },
        "day7" => { //todo
                   if part == "1" {println!("Result: {}",build_and_process_fs(lines.clone()))}
                   //if part == "2" {println!("Result: {}",start_of_message_offset(lines.clone()))}
                  },
         "day8" => {
                   if part == "1" {println!("Result: {}",visible_trees(lines.clone()))}
                   if part == "2" {println!("Result: {}",max_scenic_score(lines.clone()))}
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