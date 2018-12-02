use std::collections::HashMap;

pub fn run() {
    const PUZZLE: &str = include_str!("../inputs/day2.txt");

    let mut twocount = 0;
    let mut threecount = 0;
    let mut string_vec = Vec::new();

    for l in PUZZLE.lines() {
        let mut count_map = HashMap::new();
        let mut temp_two = 0;
        let mut temp_three = 0;
        let mut letter_count = 0;

        for c in l.chars() {
            let count = count_map.entry(c).or_insert(0);
            letter_count += c.to_digit(36).unwrap() - 10;
            *count += 1;
            if *count == 2 {
                temp_two += 1;
            } else if *count == 3 {
                temp_three += 1;
                temp_two -= 1;
            }
        }

        if temp_two > 0 {
            twocount += 1;
        }
        if temp_three > 0 {
            threecount += 1;
        }

        string_vec.push((l, letter_count));
    }

    let result = twocount * threecount;
    println!("The hash is {} * {} = {}", twocount, threecount, result);

    string_vec.sort_unstable_by(|a, b| a.1.cmp(&b.1));

    for i in string_vec.iter() {
        for k in string_vec.iter().filter(|j| j.1 >= i.1 && j.1 <= i.1 + 26) {
            let mut off_by_one = false;
            let mut d = k.0.chars();
            for c in i.0.chars() {
                let test = d.next().unwrap();
                if c != test && !off_by_one {
                    off_by_one = true;
                } else if c != test {
                    off_by_one = false;
                    break;
                }
            }
            if off_by_one {
                let mut d = k.0.chars();
                let mut result_string = String::new();
                for c in i.0.chars() {
                    if c == d.next().unwrap() {
                        result_string.push(c);
                    }
                }

                println!("Strings are {} and {} to make {}", i.0, k.0, result_string);
                return;
            }
        }
    }
}
