use std::collections::{HashMap};

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self { Self { x, y } }
}

fn get_height(map: &Vec<Vec<char>>, pos: &Position) -> i32 {
    let c = map[pos.y][pos.x];
    match c {
        'S' => 0,
        'E' => 25,
        _ => c as i32 - 97,
    }
}

fn dfs(map: &Vec<Vec<char>>, pos: Position, visited: &mut HashMap<Position, usize>) -> Option<usize> {
    let height = get_height(map, &pos);
    let steps = *visited.get(&pos).unwrap();

    if map[pos.y][pos.x] == 'E' {
        return Some(steps);
    }

    let mut next: Vec<Position> = Vec::new();
    if pos.x > 0 {
        next.push(Position::new(pos.x-1, pos.y));
    }
    if pos.y > 0 {
        next.push(Position::new(pos.x, pos.y-1));
    }
    if pos.x < map[0].len() - 1  {
        next.push(Position::new(pos.x+1, pos.y));
    }
    if pos.y < map.len() - 1 {
        next.push(Position::new(pos.x, pos.y+1));
    }

    let mut res: Option<usize> = None;
    for ne in next {
        let next_height = get_height(map, &ne);
        if (next_height-height) > 1 {
            continue;
        }

        if visited.contains_key(&ne) && *visited.get(&ne).unwrap() <= steps+1 {
            continue;
        }

        visited.insert(ne.clone(), steps+1);

        if let Some(better_res) = dfs(map, ne, visited) {
            res = Some(better_res);
        }
    }

    return res;
}

fn main() {
    let input = include_str!("../input.txt");
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    
    let start = input.replace("\n", "").find("S").unwrap();
    let start_pos = Position::new(start%map[0].len(), start/map[0].len());

    let mut visited: HashMap<Position, usize> = HashMap::new();
    visited.insert(start_pos, 0);

    print!("{}", dfs(&map, start_pos, &mut visited).unwrap())
}
