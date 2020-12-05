use aoc2020::read_lines;

fn main() {
    let lines = read_lines("data/day5_input.txt").expect("Cant read file");

    let coords: Vec<_> = lines
        .iter()
        .map(|pass| pass_to_coords(pass, 128, 8))
        .collect();
    let mut ids: Vec<_> = coords.iter().map(|(row, col)| row * 8 + col).collect();
    ids.sort();

    let part1 = ids.iter().max().unwrap();
    let part2 = find_singular_gap(ids.as_slice());

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn pass_to_coords(pass: &str, rows: usize, columns: usize) -> (usize, usize) {
    let mut high_row = rows;
    let mut low_row = 0;
    let mut high_col = columns;
    let mut low_col = 0;

    for command in pass.chars() {
        let mid_row = (high_row + low_row) / 2;
        let mid_col = (high_col + low_col) / 2;

        match command {
            'F' => high_row = mid_row,
            'B' => low_row = mid_row,
            'L' => high_col = mid_col,
            'R' => low_col = mid_col,
            _ => unreachable!("Invalid data"),
        }
    }

    (low_row, low_col)
}

fn find_singular_gap(values: &[usize]) -> usize {
    let mut previous = values[0];
    for value in &values[1..] {
        if *value == previous + 2 {
            return *value - 1;
        }
        previous = *value;
    }

    unreachable!("Error in problem");
}

#[test]
fn test_coord_conversion() {
    let result = pass_to_coords("FBFBBFFRLR", 128, 8);
    assert_eq!(result, (44, 5));
}
