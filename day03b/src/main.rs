use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    let lines = input.lines().collect::<Vec<_>>();
    let badges = lines
        .chunks(3).map(|chunk| {
            chunk.into_iter()
                .map(|l| HashSet::<_>::from_iter(l.chars()))
                .reduce(|a, b| HashSet::<_>::from_iter(a.intersection(&b).into_iter().map(|c| c.clone())))
                .unwrap()
        })
        .map(|set| set.into_iter().next().unwrap())
        .collect::<Vec<_>>();
    
    let sum:u32 = badges.into_iter()
        .map(|c| if c as u32 >= 97 {c as u32 - 97 + 1} else {c as u32 - 65 + 27})
        .sum();

    print!("{}", sum);
}
