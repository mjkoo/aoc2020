use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Map = Vec<Vec<bool>>;

fn check_slope(map: &Map, right: usize, down: usize) -> usize {
    let num_rows = map.len();
    let num_cols = map[0].len();

    let mut count = 0;
    for y in (down..num_rows).step_by(down) {
        let x = (right * y / down) % num_cols;
        if map[y][x] {
            count+= 1;
        }
    }

    count
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let filename = args.get(1).expect("No file specified");
    let file = File::open(filename).expect("Could not open input file");
    let reader = BufReader::new(file);

    let map: Map = reader
        .lines()
        .map(|l| l.unwrap().chars().map(|c| c == '#').collect())
        .collect();

    // Part 1
    println!("{:?}", check_slope(&map, 3, 1));

    // Part 2
    let slopes = vec![
        check_slope(&map, 1, 1),
        check_slope(&map, 3, 1),
        check_slope(&map, 5, 1),
        check_slope(&map, 7, 1),
        check_slope(&map, 1, 2),
    ];
    println!("{}", slopes.iter().product::<usize>());
}
