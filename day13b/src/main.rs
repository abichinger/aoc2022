use serde_json::{Value, json};
use std::cmp::{Ordering};

#[derive(Eq)]
struct PacketCmp<'a> {
    v: &'a Value
}

impl PartialOrd for PacketCmp<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PacketCmp<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<'a> PacketCmp<'a> {
    fn new(v: &'a Value) -> Self { Self { v } }

    fn cmp_arr(left_arr: &'a Value, right_arr: &'a Value) -> Ordering {

        let left = left_arr.as_array().unwrap();
        let right = right_arr.as_array().unwrap();

        for i in 0..left.len() {
            if i >= right.len() {
                return Ordering::Greater
            }
            
            let a = PacketCmp::new(&left[i]);
            let b = PacketCmp::new(&right[i]);
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

impl Ord for PacketCmp<'_> {

    fn cmp(&self, other: &Self) -> Ordering {

        let left = self.v;
        let right = other.v.clone();

        if left.is_i64() && right.is_i64() {
            return left.as_i64().unwrap().cmp(&right.as_i64().unwrap());
        }

        if left.is_i64() {
            let l = json!([left.as_i64().unwrap()]);
            return PacketCmp::cmp_arr(&l, &right);
        }
 
        if right.is_i64() {
            let r = json!([right.as_i64().unwrap()]);
            return PacketCmp::cmp_arr(&left, &r);
        }

        return PacketCmp::cmp_arr(&left, &right);
    }
}

#[derive(Eq)]
struct Packet {
    v: Value
}

impl Packet {
    fn new(v: Value) -> Self { Self { v } }

    fn from(s: &str) -> Self {
        let value: Value = serde_json::from_str(s).unwrap();
        Packet::new(value)
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        PacketCmp::new(&self.v).cmp(&PacketCmp::new(&other.v))
    }
}


fn ord_to_str(ord: Ordering) -> &'static str {
    match ord {
        Ordering::Less => "Less",
        Ordering::Equal => "Equal",
        Ordering::Greater => "Greater",
    }
}

const DIVIDER_PACKETS: &str = "
[[2]]
[[6]]
";

fn main() {
    let input = include_str!("../input.txt");
    let input_str = input.to_string() + DIVIDER_PACKETS;

    let mut packets: Vec<Packet> = input_str.split("\n")
        .filter(|l| l.trim().len() > 0)
        .map(|l| Packet::from(l))
        .collect();

    packets.sort();
    let i1 = packets.iter().position(|p| *p == Packet::from("[[2]]")).unwrap();
    let i2 = packets.iter().position(|p| *p == Packet::from("[[6]]")).unwrap();

    println!("{}", (i1+1)*(i2+1));
}
