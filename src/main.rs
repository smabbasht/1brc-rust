use std::collections::HashMap;
use std::env;
use std::fs;

// --snip--

fn main() {
    let data: HashMap<&str, Vec<i32>> = HashMap::new();
    let datapath = "../data/measurements.txt";

    println!("Reading file {}", datapath);
    let contents = fs::read_to_string(datapath).expect("Should have been able to read the file");
}
