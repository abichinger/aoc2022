use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
} 



fn main() -> std::io::Result<()> {
    if let Ok(lines) = read_lines("./calories.txt") {

        let mut max = 0;
        let mut current_elf = 0;

        for opt_line in lines {
            if let Ok(line) = opt_line {
                match line.as_str() {
                    "" => {
                        if current_elf > max {
                            max = current_elf;
                        }
                        current_elf = 0;
                    }
                    _ => {
                        let calories:i32 = line.parse().unwrap();
                        current_elf += calories;
                    }
                }
            }
        }

        println!("{}", max);
    }

    Ok(())
}
