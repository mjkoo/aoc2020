use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

fn main() {
    let args: Vec<_> = env::args().collect();
    let filename = args.get(1).expect("No file specified");
    let file = File::open(filename).expect("Could not open input file");
    let reader = BufReader::new(file);

    let lines: Vec<_> = reader.lines().map(|l| l.unwrap()).collect();
    let re = Regex::new(r"(?P<first>\d+)-(?P<second>\d+) (?P<chr>.): (?P<password>.*)").unwrap();
    let passwords: Vec<_> = lines.iter().map(|s| re.captures(&s).unwrap()).collect();

    // Part 1
    let part1 = passwords.iter().filter(|caps| {
        let min = caps["first"].parse::<usize>().unwrap();
        let max = caps["second"].parse::<usize>().unwrap();
        let chr = caps["chr"].chars().nth(0).unwrap();
        let password = &caps["password"];

        let count = password.chars().filter(|c| *c == chr).count();
        return min <= count && count <= max
    }).count();

    println!("{}", part1);

    // Part 2
    let part2 = passwords.iter().filter(|caps| {
        let first_index = caps["first"].parse::<usize>().unwrap() - 1;
        let second_index = caps["second"].parse::<usize>().unwrap() - 1;
        let chr = caps["chr"].chars().nth(0).unwrap();
        let password = &caps["password"];

        let first = password.chars().nth(first_index).unwrap() == chr;
        let second = password.chars().nth(second_index).unwrap() == chr;

        return first ^ second
    }).count();

    println!("{}", part2);
}
