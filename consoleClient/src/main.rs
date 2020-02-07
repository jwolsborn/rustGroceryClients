use std::io::{self, BufRead, Write};
use serde::{Serialize, Deserialize};
extern crate reqwest;

#[derive(Deserialize)]
struct ResBody {
    groceries: Vec<String>,
}

fn main() {
    add_item();
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
    let body = reqwest::blocking::get("http://localhost:8000/list")?.json::<ResBody>()?;

    if body.groceries.len() != 0
    {
        println!("Grocery List");
        println!("------------");
        for item in &body.groceries {
            println!("{}", item);
        }
    }
    else
    {
        println!("No items in list");
    }

    Ok(())
}

fn add_item() -> Result<(), reqwest::Error> {
    let mut item = String::new();

    let stdin = io::stdin();
    print!("Enter the item you wish to add: ");

    io::stdout().flush();
    stdin.lock().read_line(&mut item);

    let mut url = "http://localhost:8000/add/".to_string();
    url.push_str(&item.as_str());

    let client = reqwest::blocking::Client::new();
    let body = client.post(url.as_str()).send();

    match body {
        Ok(body) => { println!("Item added successfully"); Ok(())},
        Err(_) => {println!("Failed to add item!"); Ok(())}
    }
}

fn remove_item() -> Result<(), reqwest::Error> {
    let mut item = String::new();

    let stdin = io::stdin();
    print!("Enter the item you wish to remove: ");

    io::stdout().flush();
    stdin.lock().read_line(&mut item);

    let mut url = "http://localhost:8000/remove/".to_string();
    url.push_str(&item.as_str());

    let client = reqwest::blocking::Client::new();
    let body = client.put(url.as_str()).send();

    match body {
        Ok(body) => { println!("Item removed successfully"); Ok(())},
        Err(_) => {println!("Failed to remove item!"); Ok(())}
    }

}