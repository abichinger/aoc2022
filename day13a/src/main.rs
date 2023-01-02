use serde_json::{Value, json};
use std::cmp::{Ordering};

#[derive(Eq)]
struct Packet<'a> {
    v: &'a Value
}

impl PartialOrd for Packet<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Packet<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<'a> Packet<'a> {
    fn new(v: &'a Value) -> Self { Self { v } }

    fn cmp_arr(left_arr: &'a Value, right_arr: &'a Value) -> Ordering {

        let left = left_arr.as_array().unwrap();
        let right = right_arr.as_array().unwrap();

        for i in 0..left.len() {
            if i >= right.len() {
                return Ordering::Greater
            }
            
            let a = Packet::new(&left[i]);
            let b = Packet::new(&right[i]);
            let ord = a.cmp(&b);
            if ord != Ordering::Equal {
                return ord;
            }
        }

        if left.len() < right.len() {
            return Ordering::Less;
        }

        return Ordering::Equal;
    }
}

impl Ord for Packet<'_> {

    fn cmp(&self, other: &Self) -> Ordering {

        let left = self.v;
        let right = other.v.clone();

        if left.is_i64() && right.is_i64() {
            return left.as_i64().unwrap().cmp(&right.as_i64().unwrap());
        }

        if left.is_i64() {
            let l = json!([left.as_i64().unwrap()]);
            return Packet::cmp_arr(&l, &right);
        }
 
        if right.is_i64() {
            let r = json!([right.as_i64().unwrap()]);
            return Packet::cmp_arr(&left, &r);
        }

        return Packet::cmp_arr(&left, &right);
    }
}


fn ord_to_str(ord: Ordering) -> &'static str {
    match ord {
        Ordering::Less => "Less",
        Ordering::Equal => "Equal",
        Ordering::Greater => "Greater",
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let pairs = input.split("\n\n");

    let indicies: Vec<usize> = pairs
        .map(|inp| {
            let lines: Vec<&str> = inp.lines().collect();
            let left: Value = serde_json::from_str(lines[0]).unwrap();
            let right: Value = serde_json::from_str(lines[1]).unwrap();

            (left, right)
        })
        .enumerate()
        .filter(|(_, (left, right))| {
            let ord = Packet::new(left).cmp(&Packet::new(right));
            ord == Ordering::Less
        })
        .map(|(i, _)| i+1).collect();

    println!("{}", indicies.iter().map(|i| i.to_string() + ", ").collect::<String>());
    println!("{}", indicies.iter().sum::<usize>());
}
