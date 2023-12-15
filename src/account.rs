use serde::Deserialize;
use serde_json;
use std::io::{self};

#[derive(Debug, Deserialize)]
pub struct Account {
    pub user: String,
    pub balance: f64,
}

pub fn get_account() -> io::Result<Vec<Account>> {
    let json_content = include_str!("account_data.json");
    let users: Vec<Account> = serde_json::from_str(&json_content)?;
    Ok(users)
}
