use std::fs::File;
use std::io::{BufRead, BufReader};
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0, 1072);

    let filename = "/Users/stephenturcotte/Desktop/idea_generator/idea_cargo/listofideas.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        // Show the line and its number.
        //println!("{}. {}", index + 1, line);
        if index == num {
            println!("{}. {}", index + 1, line);
        }
    }
    println!("Hello world");
}




/*
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use rand::Rng;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {

    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0, 1072);
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("/Users/stephenturcotte/Desktop/idea_generator/idea_cargo/listofideas.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    }
    println!("{:?}");
}*/
















/*
use std::env;
use std::fs;
use rand::Rng;

struct Idea {
    full_idea: String,
    noun: String,
    verb: String,
}

fn main() {

    
    let full_idea = fs::read_to_string("/Users/stephenturcotte/Desktop/idea_generator/idea_cargo/listofideas.txt").expect("Unable to read file");

    let noun = fs::read_to_string("/Users/stephenturcotte/Desktop/idea_generator/idea_cargo/nouns.txt").expect("Unable to read file");

    let verbs = fs::read_to_string("/Users/stephenturcotte/Desktop/idea_generator/idea_cargo/verbs.txt").expect("Unable to read file");

    let mut rng = rand::thread_rng();
    let line1 = rng.gen_range(0, 1072);


}*/

