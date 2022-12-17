use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    let chars = input.lines().map(|l| {
        let (s1, s2) = l.split_at(l.len()/2);
        let h1 = HashSet::<_>::from_iter(s1.chars());
        let h2 = HashSet::<_>::from_iter(s2.chars());
        let c = h1.intersection(&h2).next().unwrap();
        return c.clone();
    }).collect::<Vec<_>>();
    
    let sum:u32 = chars.into_iter()
        .map(|c| if c as u32 >= 97 {c as u32 - 97 + 1} else {c as u32 - 65 + 27})
        .sum();

    print!("{}", sum);
}
