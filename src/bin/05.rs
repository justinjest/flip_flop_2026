use flip_flop::*;

#[derive(Copy, Clone)]
pub struct Road {
    direction: char,
    visited: bool,
}

impl Road {
    pub fn new(direction: char) -> Self {
        Self {
            direction,
            visited: false,
        }
    }

    pub fn visit(&mut self) {
        self.visited = true;
    }
}

fn main() {
    let example = open_file("./data/examples/05.txt");
    let data = open_file("./data/inputs/05.txt");
    run_day(example, part_1, part_2, part_3);
    run_day(data, part_1, part_2, part_3);
}

fn part_1(input: &[&str]) -> usize {
    let mut roads = vec![];
    assert!(input.len() != 0);
    let row_offset = input[0].len();

    for line in input {
        let characters: Vec<char> = line.chars().collect();
        for character in characters {
            roads.push(Road::new(character));
        }
    }

    traverse_roads(&roads, row_offset)
}

fn part_2(input: &[&str]) -> usize {
    let mut roads = vec![];
    assert!(input.len() != 0);
    let row_offset = input[0].len();

    for line in input {
        let characters: Vec<char> = line.chars().collect();
        for character in characters {
            roads.push(Road::new(character));
        }
    }

    let mut best = 0;
    for i in row_offset..roads.len() - row_offset {
        let row = i / row_offset;
        let col = i % row_offset;
        if row != 0 && row != input.len() - 1 && col != 0 && col != row_offset - 1 {
            let original = roads[i].direction;

            roads[i].direction = '^';
            let up = traverse_roads(&roads, row_offset);

            roads[i].direction = '<';
            let left = traverse_roads(&roads, row_offset);

            roads[i].direction = '>';
            let right = traverse_roads(&roads, row_offset);

            roads[i].direction = 'v';
            let down = traverse_roads(&roads, row_offset);

            let current = up.max(left).max(right).max(down);
            roads[i].direction = original;

            if current > best {
                best = current;
            }
        }
    }
    best
}

fn part_3(input: &[&str]) -> usize {
    let mut roads = vec![];
    assert!(input.len() != 0);
    let row_offset = input[0].len();

    for line in input {
        let characters: Vec<char> = line.chars().collect();
        for character in characters {
            roads.push(Road::new(character));
        }
    }

    let mut best = 0;
    for i in row_offset..roads.len() - row_offset {
        let row = i / row_offset;
        let col = i % row_offset;
        if row != 0 && row != input.len() - 1 && col != 0 && col != row_offset - 1 {
            let original = roads[i].direction;

            roads[i].direction = '^';
            let up = traverse_roads_illegally(&roads, row_offset);

            roads[i].direction = '<';
            let left = traverse_roads_illegally(&roads, row_offset);

            roads[i].direction = '>';
            let right = traverse_roads_illegally(&roads, row_offset);

            roads[i].direction = 'v';
            let down = traverse_roads_illegally(&roads, row_offset);

            let current = up.max(left).max(right).max(down);
            roads[i].direction = original;

            if current > best {
                best = current;
            }
        }
    }
    best
}

fn traverse_roads_illegally(input: &[Road], row_offset: usize) -> usize {
    let mut roads = input.to_vec();
    let mut pos = 0; // top left corner
    let mut roads_visited = 0;
    let mut illegal_turns = 0;
    loop {
        if roads[pos].visited == true {
            if illegal_turns >= 4 {
                return roads_visited;
            }
            illegal_turns += 1;
            let direction = roads[pos].direction;
            match direction {
                '^' => pos = pos + 1,
                '>' => pos = pos + row_offset,
                '<' => pos = pos - row_offset,
                'v' => pos = pos - 1,
                _ => panic!("Invalid character"),
            }

            let row = pos / row_offset;
            let col = pos % row_offset;
            if row != 0
                && row != (input.len() / row_offset) - 1
                && col != 0
                && col != row_offset - 1
            {
                return roads_visited;
            }

            roads_visited += 1;
        }
        let direction = roads[pos].direction;
        roads[pos].visit();
        roads_visited += 1;
        match direction {
            '^' => pos = pos - row_offset,
            '>' => pos = pos + 1,
            '<' => pos = pos - 1,
            'v' => pos = pos + row_offset,
            _ => panic!("Invalid character"),
        }
    }
}

fn traverse_roads(input: &[Road], row_offset: usize) -> usize {
    let mut roads = input.to_vec();
    let mut pos = 0; // top left corner
    let mut roads_visited = 0;
    loop {
        if roads[pos].visited == true {
            return roads_visited;
        }
        let direction = roads[pos].direction;
        roads[pos].visit();
        roads_visited += 1;
        match direction {
            '^' => pos = pos - row_offset,
            '>' => pos = pos + 1,
            '<' => pos = pos - 1,
            'v' => pos = pos + row_offset,
            _ => panic!("Invalid character"),
        }
    }
}
