use flip_flop::*;

fn main() {
    let data = open_file("./data/inputs/04.txt");
    run_day(data, part_1, part_2, part_3);
}

fn part_1(input: &[&str]) -> usize {
    let length = input.len();
    let cut_location = 400;
    let ground_layer = 1;
    let mut leaves = vec![0; length];
    for i in 0..length {
        if input[i].contains('o') {
            leaves[length - i] = 1;
        }
    }

    let mut number_leaves = 0;
    for i in cut_location + ground_layer + 1..length {
        // Can't harvest the layer you cut
        if leaves[i] != 0 {
            number_leaves += 1;
        }
    }
    number_leaves
}

fn part_2(input: &[&str]) -> usize {
    let length = input.len();
    let mut side = vec![false; 0];
    for i in 0..length {
        if input[i].contains("-o") {
            side.push(true);
        } else if input[i].contains("o-") {
            side.push(false);
        }
    }

    let mut number_crossings = 0;
    let mut current_side = side[side.len() - 1];
    let climbing_tree = side.into_iter().rev();
    for side in climbing_tree {
        if side != current_side {
            number_crossings += 1;
            current_side = side;
        }
    }
    number_crossings
}

fn part_3(input: &[&str]) -> usize {
    let length = input.len();
    let mut side = vec![false; 0];
    for i in 0..length {
        if input[i].contains("-o") {
            side.push(true);
        } else if input[i].contains("o-") {
            side.push(false);
        }
    }
    let mut num_workers = 0;
    let mut next = count_crossings(side);
    while next != None {
        next = count_crossings(next.unwrap());
        num_workers += 1;
    }
    num_workers
}

fn count_crossings(mut leaves: Vec<bool>) -> Option<Vec<bool>> {
    if leaves.len() == 0 {
        return None;
    }
    let mut remove_leaves = vec![];
    let mut current_side: Option<bool> = None;
    let climbing_tree = leaves.clone().into_iter().rev();
    let mut i = 0;
    for side in climbing_tree {
        i += 1;
        if current_side == None || side != current_side.unwrap() {
            current_side = Some(side);
            remove_leaves.push(i - 1);
        }
    }
    for i in remove_leaves.into_iter().rev() {
        leaves.remove(leaves.len() - 1 - i);
    }

    Some(leaves)
}
