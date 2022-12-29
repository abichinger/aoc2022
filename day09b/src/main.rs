use std::collections::HashSet;

use rusttype::Point;

struct Rope {
    knots: Vec<Point<i32>>,
}

impl Rope {
    fn new(knots: Vec<Point<i32>>) -> Self { Self { knots } }

    fn add_knot(&mut self, k: Point<i32>) {
        self.knots.push(k);
    }

    fn dx(&self, i: usize) -> i32 {
        return self.knots[i+1].x - self.knots[i].x;
    }

    fn dy(&self, i: usize) -> i32 {
        return self.knots[i+1].y - self.knots[i].y;
    }

    fn dx_abs(&self, i: usize) -> i32 {
        return self.dx(i).abs();
    }

    fn dy_abs(&self, i: usize) -> i32 {
        return self.dy(i).abs();
    }

    fn m(&mut self, x:i32, y:i32) {
        self.knots[0].x += x;
        self.knots[0].y += y;

        self.follow(0);
    }

    fn follow(&mut self, i: usize) {
        if i+1 >= self.knots.len() {
            return
        }

        if self.dx_abs(i) >= 2 {
            self.knots[i+1].x -= self.dx(i).signum();
            self.knots[i+1].y -= self.dy(i).signum();
        } else if self.dy_abs(i) >= 2 {
            self.knots[i+1].y -= self.dy(i).signum();
            self.knots[i+1].x -= self.dx(i).signum();
        }
        self.follow(i+1);
    }

}

fn parse_steps(line: &str) -> i32 {
    return line[2..].parse().unwrap();
}

fn main() {
    let input = include_str!("../input.txt");
    let mut rope = Rope::new(Vec::new());
    for _ in 0..10 {
        rope.add_knot(Point { x: 0, y: 0 })
    }

    let mut positions: HashSet<Point<i32>> = HashSet::new();

    for line in input.lines() {
        let steps = parse_steps(line);
        for _i in 0..steps {
            match &line[..1] {
                "R" => rope.m(1,0),
                "L" => rope.m(-1,0),
                "U" => rope.m(0,1),
                _ => rope.m(0,-1),
            }
            positions.insert(rope.knots[9].clone());
        }
    }

    print!("{}", positions.len())
}
