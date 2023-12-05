// https://adventofcode.com/2023/day/2

use std::fs::read_to_string;

fn min_cubes_in_bag(input: String) -> (u32, u32, u32) {
    let mut res_red: u32 = 0;
    let mut res_green: u32 = 0;
    let mut res_blue: u32 = 0;
    let subsets: Vec<&str> = input.split(';').collect();
    for subset in subsets {
        let colour_count_strings: Vec<&str> = subset.split(',').collect();
        for ccs in colour_count_strings {
            let v: Vec<&str> = ccs.trim().split(' ').collect();
            let cnt: u32 = v[0].parse().unwrap();
            if v[1].eq("red") && cnt > res_red {
                res_red = cnt;
            }
            else if v[1].eq("green") && cnt > res_green {
                res_green = cnt;
            }
            else if v[1].eq("blue") && cnt > res_blue {
                res_blue = cnt;
            }
        }
    }
    (res_red, res_green, res_blue)
}


fn main() {
    println!("--- Day 2: Cube Conundrum ---");
    let mut part_one_res: u32 = 0;
    let mut part_two_res: u32 = 0;
    for line in read_to_string("input.txt").unwrap().lines() {
        let v: Vec<&str> = line.split(": ").collect();
        let v2: Vec<&str> = v[0].split(" ").collect();
        let game_no: u32 = v2[1].parse().unwrap();
        let (min_red, min_green, min_blue) = min_cubes_in_bag(v[1].to_string());
        if min_red <= 12 && min_green <= 13 && min_blue <= 14 {
            part_one_res += game_no;
        }
        part_two_res += min_red * min_green * min_blue;
    }
    println!("Part One: {}", part_one_res);
    println!("Part Two: {}", part_two_res);
}
