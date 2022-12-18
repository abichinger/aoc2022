use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    for i in 14..input.len() {
        let marker = &input[i-14..i];
        let set: HashSet<char> = HashSet::from_iter(marker.chars());
        if set.len() == 14 {
            println!("{}, {}", marker, i);
            break;
        }
    }
}
