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

    // for node_id in root.descendants(&arena) {
    //     let node = arena.get(node_id).unwrap();
    //     println!("{}: {}", node.get().name, node.get().size)
    // }
    let res: u32 = root.descendants(&arena)
        .map(|id| arena.get(id).unwrap().get().size)
        .filter(|size| *size < 100000)
        .sum();
    
    print!("{}", res);
}
