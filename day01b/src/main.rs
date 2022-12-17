fn main() {
    let input = include_str!("../calories.txt");
    //let numbers = input.lines().map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
    
    let mut groups: Vec<Vec<&str>> = Vec::new();
    groups.push(Vec::new());
    for line in input.lines() {
        match line {
            "" => {
                groups.push(Vec::new());
            }
            _ => {
                groups.last_mut().unwrap().push(line);
            }
        }
    }
    let mut elfs = groups.iter().map(|g| {
        g.iter().map(|n| n.parse::<i32>().unwrap()).sum()
    }).collect::<Vec<i32>>();

    elfs.sort();
    let res: i32 = elfs.iter().rev().take(3).sum();

    println!("{}", res);
}
