use flip_flop::*;

fn main() {
    let data = open_file("./data/inputs/02.txt");
    run_day(data, part_1, part_2, part_3);
}

fn part_1(input: &[&str]) -> usize {
    assert!(input.len() == 1);
    let mut walls = [0; 100];
    let text = input[0];
    let mut robot_position = 1 - 1; // Instructions are one indexed but we are 0
    let mut hottest_temperature = 0;
    for direction in text.chars() {
        if direction == '<' {
            robot_position = (robot_position + 99) % 100;
            walls[robot_position] += 1;
        } else if direction == '>' {
            robot_position = (robot_position + 1) % 100;
            walls[robot_position] += 1;
        } else {
            println!("Invalid character {direction}");
        }
        if walls[robot_position] > hottest_temperature {
            hottest_temperature = walls[robot_position];
        }
    }
    println!("Hottest wall is {hottest_temperature} degrees");
    for wall_segment in 0..walls.len() {
        if walls[wall_segment] == hottest_temperature {
            println!("Located at {wall_segment}");
            return hottest_temperature * (wall_segment + 1); // Converting to index 1
        }
    }

    println!("Unable to find hottest wall segment");
    0
}

fn part_2(input: &[&str]) -> usize {
    assert!(input.len() == 1);
    let walls = [0; 100];
    let text = input[0];
    let mut robot_position = 1 - 1; // Instructions are one indexed but we are 0
    let mut wall_position = 1 - 1;
    let mut times_hit_robo_wall = 0;
    let length = text.len();
    for index in 0..length {
        let robot_direction = text.chars().nth(index).expect("Unable to parse robot");
        let wall_direction = text
            .chars()
            .nth(length - 1 - index)
            .expect("Unable to parse wall");
        if robot_direction == '<' {
            robot_position = (robot_position + 99) % 100;
        } else if robot_direction == '>' {
            robot_position = (robot_position + 1) % 100;
        } else {
            println!("Invalid character {robot_direction}");
        }
        if wall_direction == '<' {
            wall_position = (wall_position + 99) % 100;
        } else if wall_direction == '>' {
            wall_position = (wall_position + 1) % 100;
        } else {
            println!("Invalid character {wall_direction}");
        }
        if wall_position == robot_position {
            times_hit_robo_wall += 1;
        }
    }
    times_hit_robo_wall
}

fn part_3(input: &[&str]) -> usize {
    assert!(input.len() == 1);
    let mut walls = [0; 100];
    let text = input[0];
    let mut robot_position = 1 - 1; // Instructions are one indexed but we are 0
    let mut wall_offset = 1 - 1;
    let mut hottest_temperature = 0;
    let length = text.len();
    for index in 0..length {
        let robot_direction = text.chars().nth(index).expect("Unable to parse robot");
        let wall_direction = text
            .chars()
            .nth(length - 1 - index)
            .expect("Unable to parse wall");
        if wall_direction == '>' {
            wall_offset = (wall_offset + 99) % 100;
        } else if wall_direction == '<' {
            wall_offset = (wall_offset + 1) % 100;
        } else {
            println!("Invalid character {robot_direction}");
        }

        if robot_direction == '<' {
            robot_position = (robot_position + 99) % 100;
            walls[(robot_position + wall_offset) % 100] += 1;
        } else if robot_direction == '>' {
            robot_position = (robot_position + 1) % 100;
            walls[(robot_position + wall_offset) % 100] += 1;
        } else {
            println!("Invalid character {robot_direction}");
        }

        if walls[(robot_position + wall_offset) % 100] > hottest_temperature {
            hottest_temperature = walls[(robot_position + wall_offset) % 100];
        }
    }
    println!("Hottest wall is {hottest_temperature} degrees");
    for wall_segment in 0..walls.len() {
        if walls[wall_segment] == hottest_temperature {
            println!("Located at {}", wall_segment + 1);
            return hottest_temperature * (wall_segment + 1); // Converting to index 1
        }
    }

    println!("Unable to find hottest wall segment");
    0
}
