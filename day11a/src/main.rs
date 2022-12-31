use evalexpr::*;

struct Monkey {
    items: Vec<i64>,
    operation: &'static str,
    divisor: i64,
    if_true: usize,
    if_false: usize,
}

impl Monkey {
    fn new(items: Vec<i64>, operation: &'static str, divisor: i64, if_true: usize, if_false: usize) -> Self { Self { items, operation, divisor, if_true, if_false } }

    fn inspect (&mut self, mo: i64) -> (usize, i64) {
        let old = self.items.pop().unwrap();
        
        let mut ctx = HashMapContext::new();
        _ = ctx.set_value("old".to_string(), Value::from(old as i64));
        
        let new: i64 = eval_int_with_context(self.operation, &ctx).unwrap() % mo;
        //new = new / 3;
        
        if new % self.divisor == 0 {
            (self.if_true, new)
        } else {
            (self.if_false, new)
        }
    }

    fn catch(&mut self, item: i64) {
        self.items.push(item)
    }
}

fn main() {
    let input = include_str!("../input.txt");
    
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(|s| {
        let lines: Vec<_> = s.lines().into_iter().collect();
        
        let items: Vec<i64> = lines[1].split_once(":").unwrap().1.split(",").map(|i| i.trim().parse().unwrap()).collect();
        let operation: &str = lines[2].split_once("=").unwrap().1;
        let divisor: i64 = lines[3].split(" ").last().unwrap().parse().unwrap();
        let if_true: usize = lines[4].split(" ").last().unwrap().parse().unwrap();
        let if_false: usize = lines[5].split(" ").last().unwrap().parse().unwrap();

        Monkey::new(items, operation, divisor, if_true, if_false)
    }).collect::<Vec<_>>();

    let modulo = monkeys.iter().map(|m| m.divisor).product();

    let mut counter: Vec<i64> = monkeys.iter().map(|_| 0).collect();
    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                counter[i] += 1;
                let (next, item) = monkeys[i].inspect(modulo);
                monkeys[next].catch(item);
            }
        }
    }
    counter.sort();
    counter.reverse();

    println!("{}", counter.iter().map(|n| n.to_string()+ ", ").collect::<String>());
    println!("{}", counter[0]*counter[1]);

}
