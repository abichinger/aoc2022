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

    fn range_no_beacon(&self, y: i32) -> Range<i32> {
        let p = Position::new(self.s.x, y);
        let remaining = self.dist - p.manhatten_dist_to(self.s);
        if remaining < 0 {
            return 0..0
        }
        return (self.s.x-remaining)..(self.s.x+remaining+1)
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

    //let beacons :HashSet<i32> = HashSet::from_iter((&pairs).into_iter().map(|p| p.b.x));

    let y = 2000000;
    let mut no_beacon :HashSet<i32> = HashSet::new();
    for p in &pairs {
        let r = p.range_no_beacon(y);
        no_beacon.extend(r.into_iter());
    }

    //remove position with a beacon
    (&pairs).iter()
        .filter(|p| p.b.y == y)
        .for_each(|p| { no_beacon.remove(&p.b.x); });

    print!("{}", no_beacon.len());
}
