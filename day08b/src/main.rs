
type Forest = Vec<Vec<i32>>;

trait ForestTrait {
    fn is_tree_visible(&self, row: usize, col: usize) -> bool;
    fn scenic_score(&self, row: usize, col: usize) -> i32;
}

fn score(slice: &[i32], height: i32, rev: bool) -> i32 {
    let mut s = slice;
    let reversed: Vec<i32> = slice.iter().rev().map(|v| *v).collect();
    if rev {
        s = &reversed[..];
    }
    for i in 0..s.len() {
        if s[i] >= height {
            return (i+1) as i32;
        }
    }

    return s.len() as i32;
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

    fn scenic_score(&self, row: usize, col: usize) -> i32 {
        let height = self[row][col];

        let left = score(&self[row][..col], height, true);
        let right = score(&self[row][col+1..], height, false);
        let top = score(&self[..row].iter().map(|r| r[col]).collect::<Vec<_>>(), height, true);
        let bottom = score(&self[row+1..].iter().map(|r| r[col]).collect::<Vec<_>>(), height, false);

        return left*right*top*bottom;
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let forest: Forest = input.lines().map(|l| {
        l.chars().map(|c| c.to_digit(10).unwrap() as i32).collect()
    }).collect();

    // let scores: Vec<i32> = forest.iter().enumerate().flat_map(|(i, tree_row)| {
    //     tree_row.iter().enumerate().map(move |(j, _height)| forest.scenic_score(i, j))
    // }).collect();

    let rows = forest.len();
    let cols = forest.get(0).unwrap().len();

    let mut scores: Vec<i32> = Vec::new();
    for row in 0..rows {
        for col in 0..cols {
            scores.push(forest.scenic_score(row, col));
        }
    }

    print!("{}", scores.iter().max().unwrap());
}
