use flip_flop::*;

fn main() {
    let data = open_file("./data/inputs/03.txt");
    run_day(data, part_1, part_2, part_3);
}

fn part_1(input: &[&str]) -> usize {
    let mut best_password = "";
    let mut best_score = 0;
    for password in input {
        let length = password.len();
        let mut points = 0;
        let has_digit = password.chars().any(|c| c.is_ascii_digit());
        let has_lower = password.chars().any(|c| c.is_ascii_lowercase());
        let has_upper = password.chars().any(|c| c.is_ascii_uppercase());

        if has_digit {
            points += 1
        }
        if has_lower {
            points += 1
        }
        if has_upper {
            points += 1
        }

        let score = points * length;
        if score > best_score {
            best_score = score;
            best_password = password;
        }
    }

    println!("{best_password}");
    best_score
}

fn part_2(input: &[&str]) -> usize {
    let mut best_password = "";
    let mut best_score = 0;
    for password in input {
        let length = password.len();
        let not_seven = ['0', '1', '2', '3', '4', '5', '6', '8', '9'];
        let mut points = 0;

        let has_digit = password.chars().any(|c| c.is_ascii_digit());
        let has_lower = password.chars().any(|c| c.is_ascii_lowercase());
        let has_upper = password.chars().any(|c| c.is_ascii_uppercase());

        let has_seven = password.contains('7') && password.chars().all(|c| !not_seven.contains(&c));

        let has_color =
            password.contains("green") || password.contains("red") || password.contains("green");

        let substring = password.chars().collect::<Vec<char>>(); // Turn string into a vector of characters

        let longest_len = substring
            .chunk_by(|a, b| a == b) // Now works because it's a slice [char]
            .map(|chunk| chunk.len())
            .max()
            .unwrap_or(0);

        if has_digit {
            points += 1
        }
        if has_lower {
            points += 1
        }
        if has_upper {
            points += 1
        }

        if has_color {
            points *= 3
        }

        if has_seven {
            points += 7;
        }

        if longest_len >= 3 {
            points += longest_len.pow(2);
        }

        let score = points * length;

        if score > best_score {
            best_score = score;
            best_password = password;
        }
    }

    println!("{best_password}");
    best_score
}

fn part_3(input: &[&str]) -> usize {
    let mut best_character = 'a';
    let mut best_score = 0;
    let mut password = String::new();
    for c in ('a'..='z').chain('A'..='Z').chain('0'..='9') {
        let mut new_score = 0;
        for old_word in input {
            password = old_word.to_string();
            password.push(c);

            let length = password.len();
            let not_seven = ['0', '1', '2', '3', '4', '5', '6', '8', '9'];
            let mut points = 0;

            if password.chars().any(|c| c.is_ascii_digit()) {
                points += 1;
            }
            if password.chars().any(|c| c.is_ascii_lowercase()) {
                points += 1
            }

            if password.chars().any(|c| c.is_ascii_uppercase()) {
                points += 1
            }

            if password.contains('7') && password.chars().all(|c| !not_seven.contains(&c)) {
                points += 7;
            }

            let substring = password.chars().collect::<Vec<char>>(); // Turn string into a vector of characters

            let longest_len = substring
                .chunk_by(|a, b| a == b) // Now works because it's a slice [char]
                .map(|chunk| chunk.len())
                .max()
                .unwrap_or(0);

            if longest_len >= 3 {
                points += longest_len.pow(2);
            }

            if password.contains("green") || password.contains("red") || password.contains("blue") {
                points *= 3;
            }

            new_score += points * length;
        }
        if new_score > best_score {
            best_score = new_score;
            best_character = c;
        }
    }
    println!("{best_character}");
    best_score
}
