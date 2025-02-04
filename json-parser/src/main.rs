use serde_json::{Result, Value, to_string_pretty};
use std::fs;
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();                                                                                      
    if args.len() != 2 {                                                                                                                
        println!("must pass 1 parameter for the program");                                                                              
        return Ok(());                                                                                                                         
    }                                                                                                                                   

    let data = fs::read_to_string(args[1].clone())
        .expect("Should have been able to read the file");

    let v: Value = serde_json::from_str(data.as_str())?;

    let s: String = to_string_pretty(&v["people"])?;

    println!("{}", s);

    Ok(())
}
