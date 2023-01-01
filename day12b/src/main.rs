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

fn dfs(map: &Vec<Vec<char>>, pos: Position, visited: &mut HashMap<Position, usize>) {
    let height = get_height(map, &pos);
    let steps = *visited.get(&pos).unwrap();

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

    for ne in next {
        let next_height = get_height(map, &ne);
        if (next_height-height) < -1 {
            continue;
        }

        if visited.contains_key(&ne) && *visited.get(&ne).unwrap() <= steps+1 {
            continue;
        }

        visited.insert(ne.clone(), steps+1);
        dfs(map, ne, visited);
    }
}

fn main() {
    let input = include_str!("../input.txt").replace("S", "a");
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    
    //build cache
    let mut visited: HashMap<Position, usize> = HashMap::new();
    let end = input.replace("\n", "").find("E").unwrap();
    let end_pos = Position::new(end%map[0].len(), end/map[0].len());
    visited.insert(end_pos, 0);
    dfs(&map, end_pos, &mut visited);

    let res: &usize = visited.iter()
        .filter(|(pos, _)| map[pos.y][pos.x] == 'a')
        .map(|(_, v)| v)
        .min().unwrap();
    
    println!("{}", *res);
}
