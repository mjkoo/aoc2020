use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<_> = env::args().collect();
    let filename = args.get(1).expect("No file specified");
    let file = File::open(filename).expect("Could not open input file");
    let reader = BufReader::new(file);

    let nums: HashSet<_> = reader
        .lines()
        .map(|s| s.unwrap().parse::<i32>().unwrap())
        .collect();

    // Step 1
    for x in &nums {
        let y = 2020 - x;
        if *x != y && nums.contains(&y) {
            println!("{}", x * y);
            break;
        }
    }

    // Step 2
    for x in &nums {
        for y in &nums {
            let z = 2020 - x - y;
            if *x != *y && *y != z && *x != z && nums.contains(&z) {
                println!("{}", x * y * z);
                return;
            }
        }
    }
}
