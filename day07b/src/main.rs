use indextree::{Arena, NodeId};

struct Directory {
    name: &'static str,
    size: u32,
}

impl Directory {
    fn new(name: &'static str, size: u32) -> Self { Self { name, size } }
}

fn update_cd(cd: NodeId, arena: &mut Arena<Directory>, line: &'static str) -> NodeId {
    if let Some(_) = line.find("..") {
        return cd.ancestors(arena).skip(1).next().unwrap();
    }
    let name = line.split(" ").skip(2).next().unwrap();
    let dir = arena.new_node(Directory::new(name, 0));
    cd.append(dir, arena);
    return dir;
}

fn add_file(cd: NodeId, arena: &mut Arena<Directory>, line: &'static str) {
    let size: u32 = line.split(" ").next().unwrap().parse().unwrap();

    for node_id in cd.ancestors(arena).collect::<Vec<_>>() {
        let dir = arena.get_mut(node_id).unwrap().get_mut();
        dir.size += size;
    }
}

const DISC_SPACE: u32 = 70000000;
const UPDATE_SIZE: u32 = 30000000;

fn main() {
    let input = include_str!("../input.txt");
    let mut arena: Arena<Directory> = Arena::new();
    let root = arena.new_node(Directory::new("root", 0));
    let mut cd = root.clone();

    for line in input.lines() {
        match &line[..1]{
            "$" => {
               match &line[2..4] {
                "cd" => {
                    cd = update_cd(cd, &mut arena, line);
                }
                _ => {}
               };
            },
            _ => {
                match &line[0..3] {
                    "dir" => {}
                    _ => {
                        add_file(cd.clone(), &mut arena, line);
                    }
                }
            }
        };
    }

    let free = DISC_SPACE - arena.get(root).unwrap().get().size;
    let free_up: i32 = UPDATE_SIZE as i32 - free as i32;

    let res: i32 = root.descendants(&arena)
        .map(|id| arena.get(id).unwrap().get().size)
        .map(|size| (size as i32)-free_up)
        .filter(|size| *size > 0)
        .min().unwrap();
    
    print!("{}", res+free_up);
}
