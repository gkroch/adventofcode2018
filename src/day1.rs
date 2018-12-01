use std::collections::BTreeSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn run() {
    let file = File::open("../inputs/day1.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut sum: i32 = 0;
    let mut seen_sums = BTreeSet::new();
    let mut num_vec = Vec::new();
    seen_sums.insert(0);

    let mut no_break = true;

    for l in lines {
        let num: i32 = l.unwrap().parse().unwrap();
        sum += num;
        num_vec.push(num);
        if !seen_sums.contains(&sum) {
            seen_sums.insert(sum);
        } else {
            println!("Repeated frequency of {}", sum);
        }
    }

    println!("Received final frequency of {}", sum);

    while no_break {
        for entry in &num_vec {
            sum += entry;
            if !seen_sums.contains(&sum) {
                seen_sums.insert(sum);
            } else {
                println!("Repeated frequency of {}", sum);
                no_break = false;
                break;
            }
        }
    }
}
