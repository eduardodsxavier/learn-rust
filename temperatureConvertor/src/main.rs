use std::io;

fn main() {
    println!("mesure to convert: \n1)celsius \n2)Fahrenheit \n3)Kelvin");    

    let mut given_mesure = String::new();

    let _ = io::stdin().read_line(&mut given_mesure);

    if given_mesure.trim() != "1" && given_mesure.trim() != "2" && given_mesure.trim() != "3"  {
        println!("not valid");
        return;
    }

    println!("mesure to be converted: \n1)celsius \n2)Fahrenheit \n3)Kelvin");    

    let mut aim_mesure = String::new();

    let _ = io::stdin().read_line(&mut aim_mesure);

    if aim_mesure.trim() == given_mesure.trim() {
        println!("both mesures are the same");
        return;
    }

    println!("give temp to be converted: ");    

    let mut temp = String::new();

    let _ = io::stdin().read_line(&mut temp);

    if given_mesure.trim() == "1" && aim_mesure.trim() == "2" {
        println!("{} celsius = {} fahrenheit", temp.trim(), temp.trim().parse::<f32>()
            .expect("invalid temperature") * 1.8 + 32.0);
        return;
    }

    if given_mesure.trim() == "1" && aim_mesure.trim() == "3" {
        println!("{} celsius = {} kelvin", temp.trim(), temp.trim().parse::<f32>()
            .expect("invalid temperature") + 273.15);
        return;
    }

    if given_mesure.trim() == "2" && aim_mesure.trim() == "1" {
        println!("{} fahrenheit = {} celsius", temp.trim(), (temp.trim().parse::<f32>()
            .expect("invalid temperature") - 32.0) / 1.8);
        return;
    }

    if given_mesure.trim() == "2" && aim_mesure.trim() == "3" {
        println!("{} fahrenheit = {} kelvin", temp.trim(), (temp.trim().parse::<f32>()
            .expect("invalid temperature") - 32.0) / 1.8 + 273.15);
        return;
    }

    if given_mesure.trim() == "3" && aim_mesure.trim() == "1" {
        println!("{} kelvin = {} celsius", temp.trim(), temp.trim().parse::<f32>()
            .expect("invalid temperature") - 273.15);
        return;
    }

    println!("{} kelvin = {} fahrenheit", temp.trim(), (temp.trim().parse::<f32>()
        .expect("invalid temperature") - 273.15) * 1.8 + 32.0);
}
