use flip_flop::*;

fn main() {
    let example = open_file("./data/inputs/01.txt");
    run_day(example, part_1, part_2, part_3);
}

fn part_1(input: &[&str]) -> usize {
    let mut total = 0;
    for i in input {
        let val: usize = i.parse().expect("Failed to parse string to usize");
        if val < 60 {
            total += 60 - val;
        }
    }
    total
}

fn part_2(input: &[&str]) -> usize {
    let mut total = 0;
    for i in input {
        let val: usize = i.parse().expect("Failed to parse string to usize");
        if val < 60 {
            total += 60 - val;
        } else {
            total += (val - 60) * 5;
        }
    }
    total
}

fn part_3(input: &[&str]) -> usize {
    let mut total = 0;
    let index = input.len() / 2;
    assert!(index % 2 == 0); // Don't think this will work if it's odd
    for i in 0..index {
        let current_temp: usize = input[i].parse().expect("Failed to parse string to usize");
        let expected_temp: usize = input[i + index]
            .parse()
            .expect("Failed to parse string to usize");
        if current_temp < expected_temp {
            total += expected_temp - current_temp;
        } else {
            total += (current_temp - expected_temp) * 5;
        }
    }
    total
}
