use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    println!("{}", std::env::current_dir().unwrap().display());

    let file = File::open("elves.txt")?;
    let reader = BufReader::new(file);

    let mut elves: Vec<Vec<String>> = vec![vec![]];
    for line in reader.lines() {
        match line {
            Ok(s) => {
                if s == "" {
                    elves.push(vec![]);
                } else {
                    let len = elves.len();
                    elves[len - 1].push(s);
                }
            }
            _error => (),
        }
    }
    println!("{}",elves.len());
    let mut sums: Vec<i32> = vec![];
    for l in elves {
        let mut sum = 0;
        for e in l {
            sum += e.parse::<i32>().unwrap();
        }
        sums.push(sum);
        println!("Sum {}", sum);
    }
    let max_value = sums.iter().max();
    match max_value {
        Some(max)  => println!("Max value: {}", max),
        None => println!("Vector is empty")
    }

    //Now also get the top three and their sums
    sums.sort();
    let len = sums.len();
    println!("{}", sums[len - 1] + sums[len - 2] + sums[len - 3]);

    Ok(())
}
