use std::{ops::{Range}, collections::HashSet};

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y:i32) -> Self {
        Self {x, y}
    }

    fn manhatten_dist_to(&self, other: Position) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn from(s: &str) -> Self {
        let (s0, s1) = s.split_once(",").unwrap();
        let x :i32 = s0.split_once("=").unwrap().1.parse().unwrap();
        let y :i32 = s1.split_once("=").unwrap().1.parse().unwrap();
        Position::new(x, y)
    }
}

struct SensorBeacon {
    s: Position,
    b: Position,
    dist: i32,
}

impl SensorBeacon {
    fn new(s: Position, b: Position, dist: i32) -> Self { Self { s, b, dist } }

    fn iter(&self) -> SensorLoop {
        SensorLoop::new(self.s, self.dist+1)
    }
}

struct SensorLoop {
    s: Position,
    dist: i32,
    i: i32,
    j: i32,
}

impl SensorLoop {
    fn new(s: Position, dist: i32) -> Self { Self { s, dist, i:0, j:0 } }
}

impl Iterator for SensorLoop {
    type Item = Position;

    fn next(&mut self) -> Option<Position> {
        let i = self.i;
        let dist = self.dist;
        let x = self.s.x;
        let y = self.s.y;

        if i >= dist {
            return None
        }
        let res = match self.j {
            0 => Position::new(x+dist-i, y+i),
            1 => Position::new(x-dist+i, y+i),
            2 => Position::new(x+dist-i, y-i),
            3 => Position::new(x-dist+i, y-i),
            _ => panic!()
        };

        if self.j == 3 {
            self.i += 1;
        }

        self.j = (self.j+1)%4;

        return Some(res);
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let pairs: Vec<SensorBeacon> = input.lines().map(|l| {
        let (s, b) = l.split_once(":").unwrap();
        let sp = Position::from(s);
        let bp = Position::from(b);
        return SensorBeacon::new(sp, bp, sp.manhatten_dist_to(bp))
    }).collect();

    let limit = 4000000;
    
    for p in &pairs {
        for pos in p.iter() {
            if pos.x < 0 || pos.y < 0 {
                continue;
            }
            if pos.x > limit || pos.y > limit {
                continue;
            }

            let mut discovered = false;
            for p2 in &pairs {
                let dist = pos.manhatten_dist_to(p2.s);
                if dist <= p2.dist {
                    discovered = true;
                    break;
                }
            }
            if !discovered {
                println!("x: {}, y: {}, tuning frequency: {}", pos.x, pos.y, (pos.x as i64)*4000000+(pos.y as i64));
                return
            }
        }
    }
}
