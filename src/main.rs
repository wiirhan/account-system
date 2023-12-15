use serde::Deserialize;
use serde_json;
use std::io::{self};

#[derive(Debug, Deserialize)]
struct Account {
    user: String,
    balance: f64,
}

fn get_account() -> io::Result<Vec<Account>> {
    let json_content = include_str!("account_data.json");
    let users: Vec<Account> = serde_json::from_str(&json_content)?;
    Ok(users)
}

fn main() {
    let account_system = get_account().unwrap();

    println!("Initial State:");
    for account in &account_system {
        println!("{} {}", account.user, account.balance);
    }
}
