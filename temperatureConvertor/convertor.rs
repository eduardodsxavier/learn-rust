use std::io;

fn main() {
    println!("given mesure: ");    

    let mut given_mesure = String::new();

    let _ = io::stdin().read_line(&mut given_mesure);

    if given_mesure.trim() == "celcius" {
        println!("200");
    }
    else { println!("500"); }
}
