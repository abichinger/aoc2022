struct CPU<T> 
where
    T: FnMut(i32, i32),
{
    cycle: i32,
    on_cycle: T,
    x: i32,
}

impl<T> CPU<T>
where
    T: FnMut(i32, i32),
{
    fn new(cycle: i32, closure: T, x: i32) -> Self { Self { cycle, on_cycle: closure, x } }

    fn set_cycle(&mut self, cycle: i32) {
        self.cycle = cycle;
        (self.on_cycle)(self.cycle, self.x); 
    }

    fn inc_cycle(&mut self) {
        self.set_cycle(self.cycle+1);
    }
    
    fn noop(&mut self) {
        self.inc_cycle();
    }

    fn addx(&mut self, v: i32) {
        self.inc_cycle();
        self.inc_cycle();
        self.x += v;
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let mut strengths: Vec<i32> = Vec::new();

    {
        let closure = |cycle, x| {
            if (cycle-20) % 40 != 0 {
                return;
            }
            strengths.push(cycle*x);
        };
        
        let mut cpu = CPU::new(0, closure, 1);

        for line in input.lines() {
            match &line[..1] {
                "n" => cpu.noop(),
                _ => {
                    let v = line[5..].parse().unwrap();
                    cpu.addx(v);
                },
            }
        }
    }

    println!("{}", strengths.iter().sum::<i32>());
}
