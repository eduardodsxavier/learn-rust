use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 && args.len() != 4 {
        println!("invalid number of parameters");
        return Ok(());
    }

    if args[1].trim() == "-G" {
        let _ = get_request(args[2].trim().to_string())
            .await?;
    }


    if args[1].trim() == "-P" {
        let _ = post_request(args[2].trim().to_string(), args[2].trim().to_string())
            .await?;
    }

    Ok(())
}

async fn get_request(url: String)-> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::get(url)
        .await?
        .text()
        .await?;

    println!("body = {body:?}");
    Ok(())
}

async fn post_request(url: String, body: String)-> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client.post(url)
        .body(body)
        .send()
        .await?;

    println!("res = {res:?}");
    Ok(())
}
