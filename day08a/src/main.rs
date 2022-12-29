
type Forest = Vec<Vec<i32>>;

trait ForestTrait {
    fn is_tree_visible(&self, row: usize, col: usize) -> bool;
}

impl ForestTrait for Forest {
    fn is_tree_visible(&self, row: usize, col: usize) -> bool {
        let height = self[row][col];

        let left = &self[row][..col];
        let right = &self[row][col+1..];
        let top = self[..row].iter().map(|r| r[col]).collect::<Vec<_>>();
        let bottom = self[row+1..].iter().map(|r| r[col]).collect::<Vec<_>>();

        let directions = vec![left, right, &top[..], &bottom[..]];

        for d in directions {
            if height > *d.iter().max().unwrap_or(&-1) {
                return true
            }
        }

        return false;
    }
}

fn main() {
    let input = include_str!("../example_input.txt");
    let forest: Forest = input.lines().map(|l| {
        l.chars().map(|c| c.to_digit(10).unwrap() as i32).collect()
    }).collect();

    let rows = forest.len();
    let cols = forest.get(0).unwrap().len();

    let mut counter = 0;
    for row in 0..rows {
        for col in 0..cols {
            if forest.is_tree_visible(row, col) {
                counter += 1;
            }
        }
    }

    print!("{}", counter);
}
