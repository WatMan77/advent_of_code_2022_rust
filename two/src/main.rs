use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn read() -> Option<BufReader<File>> {
    let file = File::open("rocks.txt");
    match file {
        Ok(f) => return Some(BufReader::new(f)),
        _error => return None,
    }
}

fn did_i_win(first: String, second: String) -> bool {
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

    let points = HashMap::from([("A", 1), ("B", 2), ("C", 3), ("X", 1), ("Y", 2), ("Z", 3)]);
    let first = &situation[0..1]; //Me
    let second = &situation[2..3]; //Opponent

    print!("{} {}", first, second);

    if first == "A" && second == "X"
        || first == "B" && second == "Y"
        || first == "C" && second == "Z"
    {
        // Draw. Return 3 + the score of the option you chose
        println!(" It's a draw. {} + 3", points[second]);
        return points[second] + 3;
    }

    if did_i_win(String::from(first), String::from(second)) {
        println!(" You win! {} + 6", points[second]);
        return points[second] + 6;
    } else {
        println!(" You lose. {}", points[second]);
        return points[second];
    }
}

fn secretrps(situation: String) -> i32 {
    //X -> need to lose
    //Y -> need to draw
    //Z -> need to win

    let opp = &situation[0..1];
    let result = &situation[2..3];

    let points = HashMap::from([("A", 1), ("B", 2), ("C", 3), ("X", 1), ("Y", 2), ("Z", 3)]);
    let losses = HashMap::from([("A", 3), ("B", 1), ("C", 2)]);
    let wins = HashMap::from([("A", 2), ("B", 3), ("C", 1)]);

    let opponent: &str = &opp;

    // Need to lose
    if result == "X" {
        let point: i32 = losses[opponent];
        return point;
    }
    // Need to draw
    if result == "Y" {
        return points[opponent] + 3;
    }
    //Need to win   
    if result == "Z" {
        let point = wins[opponent];
        return point + 6;
    }

    0
}

fn main() {
    let reader = read();
    let re: BufReader<File>;
    match reader {
        None => return,
        Some(r) => re = r,
    }

    let mut sum = 0;

    for line in re.lines() {
        match line {
            Ok(l) => sum += secretrps(l),
            _error => (),
        }
    }
    println!("Score is: {}", sum);
}
