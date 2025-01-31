use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("must pass 1 parameter for the program");
        return;
    }

    let contents = fs::read_to_string(args[1].clone())
        .expect("Should have been able to read the file");

    let mut words_count = HashMap::new();

    for word in contents.split_whitespace() {
        *words_count.entry(word).or_insert(0) += 1;
    }

    println!("{words_count:?}");
}
