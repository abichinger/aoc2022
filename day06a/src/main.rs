use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    for i in 4..input.len() {
        let marker = &input[i-4..i];
        let set: HashSet<char> = HashSet::from_iter(marker.chars());
        if set.len() == 4 {
            println!("{}, {}", marker, i);
            break;
        }
    }
}
