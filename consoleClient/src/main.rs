use std::io::{self, BufRead, Write};
use serde::{Serialize, Deserialize};
extern crate reqwest;

#[derive(Deserialize)]
struct resBody {
    groceries: Vec<String>,
}

fn main() {
    get_list();
    run();
}

fn run() {
    let mut selection: String = String::new();

    while selection.trim() != "4".to_string() {
        selection = String::new();
        println!("Make a selection:");
        println!("1) Get grocery list");
        println!("2) Add Item");
        println!("3) Remove Item");
        println!("4) Exit");

        let stdin = io::stdin();
        print!("Enter your selection: ");

        io::stdout().flush();
        stdin.lock().read_line(&mut selection);
    }
}

fn get_list() -> Result<(), reqwest::Error> {
    let body = reqwest::blocking::get("http://localhost:8000/list")?.json::<resBody>()?;
    body.groceries.iter().map(|x| println!("{}", x));
    Ok(())
}