use std::collections::HashSet;

use rusttype::Point;

struct Rope {
    head: Point<i32>,
    tail: Point<i32>,
}

impl Rope {
    fn new(head: Point<i32>, tail: Point<i32>) -> Self { Self { head, tail } }

    fn dx(&self) -> i32 {
        return self.tail.x - self.head.x;
    }

    fn dy(&self) -> i32 {
        return self.tail.y - self.head.y;
    }

    fn dx_abs(&self) -> i32 {
        return self.dx().abs();
    }

    fn dy_abs(&self) -> i32 {
        return self.dy().abs();
    }

    fn right(&mut self) {
        self.head.x += 1;
        if self.dx_abs() >= 2 {
            self.tail.x += 1;
            self.tail.y = self.head.y;
        }
    }

    fn left(&mut self) {
        self.head.x -= 1;
        if self.dx_abs() >= 2 {
            self.tail.x -= 1;
            self.tail.y = self.head.y;
        }
    }

    fn up(&mut self) {
        self.head.y += 1;
        if self.dy_abs() >= 2 {
            self.tail.y += 1;
            self.tail.x = self.head.x;
        }
    }

    fn down(&mut self) {
        self.head.y -= 1;
        if self.dy_abs() >= 2 {
            self.tail.y -= 1;
            self.tail.x = self.head.x;
        }
    }
}

fn parse_steps(line: &str) -> i32 {
    return line[2..].parse().unwrap();
}

fn main() {
    let input = include_str!("../input.txt");
    let mut rope = Rope::new(Point { x: 0, y: 0 }, Point { x: 0, y: 0 });
    let mut positions: HashSet<Point<i32>> = HashSet::new();

    for line in input.lines() {
        let steps = parse_steps(line);
        for _i in 0..steps {
            match &line[..1] {
                "R" => rope.right(),
                "L" => rope.left(),
                "U" => rope.up(),
                _ => rope.down(),
            }
            positions.insert(rope.tail.clone());
        }
    }

    print!("{}", positions.len())
}
