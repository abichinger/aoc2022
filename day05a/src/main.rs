fn parse_crates(s: &str, n: usize) -> Vec<Vec<char>> {

    let mut stacks: Vec<Vec<char>> = Vec::new();
    (0..n).for_each(|_| stacks.push(Vec::new()));

    s.lines().for_each(|l| l
        .chars()
        .collect::<Vec<char>>()
        .chunks(4)
        .enumerate()
        .for_each(|(i, c)| {
            if !c.starts_with(&['[']) {
                return
            }
            stacks[i].insert(0, c[1])
        })
    );

    return stacks;
}

fn exec_move(s: &str, mut stacks: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let split = s.split(" ").collect::<Vec<_>>();
    let n: usize = split.get(1).unwrap().parse().unwrap();
    let src: usize = split.get(3).unwrap().parse().unwrap();
    let dest: usize = split.get(5).unwrap().parse().unwrap();

    for _ in 0..n {
        let c = stacks[src-1].pop().unwrap();
        stacks[dest-1].push(c);
    }

    return stacks
}

fn main() {
    let input = include_str!("../input.txt");
    let (crates, moves) = input.split_once("\n\n").unwrap();

    let mut stacks = parse_crates(crates, 9);

    // for stack in stacks.as_slice() {
    //     for c in stack {
    //         print!("{}", c);
    //     }
    //     println!();
    // }

    for l in moves.lines() {
        stacks = exec_move(l, stacks);
    }

    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }
    println!();

}
