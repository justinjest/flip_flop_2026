use std::fmt::Debug;
use std::fs::read_to_string;
use std::time::Instant;

pub fn open_file(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

pub fn run_day<T: Debug>(
    file: Vec<String>,
    part1: fn(&[&str]) -> T,
    part2: fn(&[&str]) -> T,
    part3: fn(&[&str]) -> T,
) {
    let file_slice: Vec<&str> = file.iter().map(|s| s.as_str()).collect();

    let now = Instant::now();
    let p1 = part1(&file_slice);
    println!("part 1 is {:?}", p1);

    let elapsed = now.elapsed();
    println!("Took {} mils", elapsed.as_millis());

    let p2 = part2(&file_slice);
    println!("Part 2 is {:?}", p2);

    let elapsed = now.elapsed();
    println!("Took {} mils", elapsed.as_millis());

    let p3 = part3(&file_slice);
    println!("Part 3 is {:?}", p3);

    let elapsed = now.elapsed();
    println!("Took {} mils", elapsed.as_millis());
}
