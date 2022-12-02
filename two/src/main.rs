use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;



fn read() -> Option<BufReader<File>> {
    let file = File::open("rocks.txt");
    match file {
        Ok(f) => return Some(BufReader::new(f)),
        _error => return None,
    }
}

fn did_i_win(first: String, second: String) -> bool{
    //X beats C
    //Y beats A
    //Z beats B
    if first == "C" && second == "X" {
        return true;
    }
    if first == "A" && second == "Y" {
        return true;
    }
    if first == "B" && second == "Z" {
        return true;
    }

    return false;
}

fn rps(situation: String) -> i32 {
    //A and X = Rock -> 1 point
    //B and Y = Paper -> 2 points
    //C and Z = Scissors -> 3 points

    let points = HashMap::from([
        ("A", 1),
        ("B", 2),
        ("C", 3),
        ("X", 1),
        ("Y", 2),
        ("Z", 3)

    
    ]);
    let first = &situation[0..1]; //Me
    let second = &situation[2..3]; //Opponent

    print!("{} {}", first, second);

    if first == "A" && second == "X" || first == "B" && second == "Y" || first == "C" && second == "Z" {
        // Draw. Return 3 + the score of the option you chose
        println!(" It's a draw. {} + 3", points[second]);
        return points[second] + 3;
    }

    if did_i_win(String::from(first), String::from(second)){
        println!(" You win! {} + 6", points[second]);
        return points[second] + 6;        
    } else {
        println!(" You lose. {}", points[second]);
        return points[second];
    }
}

fn main() {
    let reader = read();
    let re: BufReader<File>;
    match reader {
        None => return,
        Some(r) => re = r
    }

    let mut sum = 0;

    for line in re.lines() {
        match line {
            Ok(l) => sum += rps(l),
            _error => (),
        }
        
    }
    println!("Score is: {}", sum);
}
