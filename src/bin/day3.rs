use aoc2020::advent_main;
use itertools::Itertools;

advent_main!(day3, "data/day3_input.txt");

fn day3(lines: &Vec<String>) -> (usize, usize) {
    let grid = Grid::from_lines(&lines);

    let part1 = traverse_and_count(&grid, 3, 1);
    let part2: usize = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(dx, dy)| traverse_and_count(&grid, *dx, *dy))
        .product();
    (part1, part2)
}

fn traverse_and_count(grid: &Grid, delta_x: usize, delta_y: usize) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut count = 0;
    while y < grid.true_height {
        if grid.get(x, y) == Slot::Tree {
            count += 1;
        }
        x += delta_x;
        y += delta_y;
    }
    count
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Slot {
    Tree,
    Empty,
}

struct Grid {
    data: Vec<Slot>,
    true_width: usize,
    true_height: usize,
}

impl Grid {
    fn from_lines(lines: &Vec<String>) -> Grid {
        let true_width = lines[0].len();
        let true_height = lines.len();

        let data = (0..true_height)
            .cartesian_product(0..true_width)
            .map(|(y, x)| {
                if lines[y].chars().nth(x) == Some('#') {
                    Slot::Tree
                } else {
                    Slot::Empty
                }
            })
            .collect();

        Grid {
            data,
            true_width,
            true_height,
        }
    }

    fn get(&self, x: usize, y: usize) -> Slot {
        let mx = x % self.true_width;
        let my = y % self.true_height;

        self.data[my * self.true_width + mx]
    }
}

#[test]
fn test_grid() {
    let grid = Grid::from_lines(&read_lines("data/day3_input.txt").unwrap());

    assert_eq!(grid.true_width, 31);

    assert_eq!(grid.get(0, 0), Slot::Empty);
    assert_eq!(grid.get(30, 0), Slot::Tree);
    assert_eq!(grid.get(30 + (31 * 5), 0), Slot::Tree);
    assert_eq!(grid.get(22, 15), Slot::Tree);
}
