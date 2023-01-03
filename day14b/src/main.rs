use std::collections::{HashMap, HashSet};
use std::cmp::{min, max};

fn interpolate((x1,y1): (usize, usize), (x2,y2): (usize, usize),) -> Vec<(usize, usize)> {   
    (min(x1, x2)..max(x1, x2)+1).flat_map(|x| {
        (min(y1, y2)..max(y1, y2)+1).map(move |y| (x, y))
    }).collect()
}

fn interpolate_line(corners: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut points: Vec<(usize, usize)> = Vec::new();
    for i in 1..corners.len() {
        let mut vec = interpolate(corners[i-1], corners[i]);
        points.append(&mut vec);
    }
    return points;
}

fn parse_line(line: &str) -> Vec<(usize, usize)> {
    line.split(" -> ")
        .map(|p| p.split_once(",").unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect()
}

struct Cave {
    map: HashMap<usize, HashSet<usize>>,
    bounds: Bounds,
}

impl Cave {
    fn new() -> Self { 
        Self { 
            map: HashMap::new(), 
            bounds: Bounds::from(&[(500, 0)]) 
        }
    }

    fn insert_point(&mut self, (x, y): (usize, usize)) {
        let set = match self.map.get_mut(&x) {
            Some(set) => set,
            None => {
                self.map.insert(x, HashSet::new());
                self.map.get_mut(&x).unwrap()
            },
        };
        set.insert(y);
    }

    fn insert_line(&mut self, corners: Vec<(usize, usize)>) {
        self.bounds.extend(&corners);

        let points = interpolate_line(corners);
        for p in points {
            self.insert_point(p);
        }
    }

    fn is_blocked(&self, (x, y): (usize, usize)) -> bool {        
        let ground = self.bounds.max_y + 2;
        if y >= ground {
            return true
        }
        
        match self.map.get(&x) {
            Some(set) => set.contains(&y),
            None => false,
        }
    }

    fn insert_sand(&mut self) -> bool {
        let mut x: usize = 500;
        let mut y: usize = 0;

        if self.is_blocked((x,y)) {
            return false
        }

        loop {
            if !self.is_blocked((x,y+1)) {
                y+=1;
            } else if !self.is_blocked((x-1,y+1)) {
                x-=1;
                y+=1;
            } else if !self.is_blocked((x+1,y+1)) {
                x+=1;
                y+=1;
            } else {
                break;
            }
        }
        self.insert_point((x, y));
        return true
    }

}

struct Bounds {
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
}

impl Bounds {
    fn from(points: &[(usize, usize)]) -> Bounds {
        if points.len() == 0 {
            panic!()
        }
        
        let mut b = Bounds {
            min_x: points[0].0,
            max_x: points[0].0,
            min_y: points[0].1,
            max_y: points[0].1,
        };

        b.extend(&points[1..]);
        
        return b;
    }

    fn extend(&mut self, points: &[(usize, usize)]) {
        for &(x, y) in points {
            self.min_x = if x < self.min_x { x } else { self.min_x };
            self.max_x = if x > self.max_x { x } else { self.max_x };
            self.min_y = if y < self.min_y { y } else { self.min_y };
            self.max_y = if y > self.max_y { y } else { self.max_y };
        }
    }

    fn is_inside(&self, (x,y): (usize, usize)) -> bool {
        return x >= self.min_x && x <= self.max_x &&
            y >= self.min_y && y <= self.max_y
    }

    fn is_outside(&self, p: (usize, usize)) -> bool {
        !self.is_inside(p)
    }

    fn to_string(&self) -> String {
        format!("({}, {}), ({}, {})", self.min_x, self.min_y, self.max_x, self.max_y)
    }

}

fn main() {
    let input = include_str!("../input.txt");
    let mut cave: Cave = Cave::new();

    for line in input.lines() {
        let corners = parse_line(line);
        cave.insert_line(corners);
    }

    let mut counter = 0;
    while cave.insert_sand() {
        counter+=1;
    }

    println!("{}", counter);
}
