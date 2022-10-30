use reqwest::StatusCode;
use std::process;

pub fn ask_continue() {
    let mut line = String::new();
    println!("Contniue?");
    let answer = std::io::stdin().read_line(&mut line).unwrap();
    println!("{}", line);
    if answer.to_string() != "yes" {
        println!("exiting due to user input");
    }
}

pub fn check_url(endpoint: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--> Check URL: {}", endpoint);

    let response = reqwest::blocking::get(&endpoint)?;
    let verbose = false;

    match response.status() {
        StatusCode::OK => println!("200 ok"),
        s => {
            println!("Recieved response status: {:?}", s);
            process::exit(1);
        }
    }

    if verbose {
        println!("response {:#?}", response);
    }

    Ok(())
}

pub fn echo(service: String) {
    println!("echo {}", service);
}

pub fn test(service: String) {
    println!("test {}", service);
    ask_continue();
    println!("test2 {}", service);
}
