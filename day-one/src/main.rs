use std::{collections::HashMap, fs::File};
use counter::Counter;
use csv::Trim;

type Record = Vec<i64>;

fn parse_csv_input() -> (Vec<i64>, Vec<i64>) {
    let mut table_one: Vec<i64> = Vec::new();
    let mut table_two: Vec<i64> = Vec::new();

    let file_path = "/Users/chipcarnes/Projects/advent-of-code/day-one/src/input.csv".to_string();
    let file:File;
    match File::open(file_path) {
        Ok(f) => {
            file = f;
        }
        Err(e) => {
            panic!("{}", e)
        }
    }

    let mut csv_reader = csv::ReaderBuilder::new().has_headers(false).trim(Trim::All).from_reader(file);
    for result in csv_reader.deserialize() {
        match result {
            Ok(r) => {
                let record: Record = r;
                table_one.push(record[0]);
                table_two.push(record[1]);
            }
            Err(e) => {
                panic!("{}", e)
            }
        }
    }
    (table_one, table_two)
}

fn part_one() -> i64 {
    let (mut t_one, mut t_two) = parse_csv_input();

    t_one.sort();
    t_two.sort();

    let total_distance: i64 = t_one.iter()
        .zip(t_two.iter())
        .map(|(a, b)| {
            (a - b).abs()
        })
        .sum();

    total_distance
}

fn part_two() -> i64 {
    let (t_one, t_two) = parse_csv_input();

    let distance_counter = t_two.iter().collect::<Counter<_>>();
    let mut singularity_score: i64 = 0;
    t_one.iter().for_each(|num| {
        singularity_score += num * (*distance_counter.get(num).unwrap_or(&0) as i64);
    });

    // let mut count_map: HashMap<_, _> = t_one.iter().map(|x| (x.clone(), 0)).collect();

    // t_two.iter().for_each(|y| {
    //     count_map.entry(*y).and_modify(|entry| *entry += 1);
    // });

    // let mut singularity_score: i64 = 0;

    // count_map.into_iter().for_each(|(k, v)| {
    //     let base_multiplier = k * v;

    //     singularity_score += base_multiplier
    // });

    singularity_score
}

fn main() {
    println!("{}", part_one());
    println!("{:?}", part_two());
}
