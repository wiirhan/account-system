mod account;

use account::{AccountSystem, Transaction, Transfer};
use serde_json;
use std::io::{self};

fn main() {
    let mut account_system = AccountSystem::new();
    account_system.get_account().unwrap();

    let transactions = get_transaction().unwrap();

    let max_profit_transaction = account_system.find_max_profit_transaction(&transactions);

    println!("Maximum profit transactions:");
    println!("{:?}", max_profit_transaction);
    if let Err(e) = account_system.execute_transaction(&max_profit_transaction) {
        eprintln!("Error executing transaction: {}", e);
    }

    account_system.display_accounts();
}

fn get_transaction() -> io::Result<Vec<Transaction>> {
    let json_content = include_str!("transactions.json");
    match serde_json::from_str::<Vec<Vec<Transfer>>>(&json_content) {
        Ok(transaction) => Ok(transaction
            .into_iter()
            .map(|transfers| Transaction { transfers })
            .collect()),
        Err(err) => {
            eprintln!("Error deserializing transaction data: {}", err);
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid transaction data",
            ))
        }
    }
}
