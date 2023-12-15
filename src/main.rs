use serde::Deserialize;
use serde_json;
use std::io::{self};

#[derive(Debug)]
struct Transfer {
    from: String,
    to: String,
    amount: f64,
    fee: f64,
}

#[derive(Debug)]
struct Transaction {
    transfers: Vec<Transfer>,
}

struct AccountSystem {
    accounts: Vec<Account>,
    system_balance: f64,
}

#[derive(Debug, Deserialize, Clone)]
struct Account {
    user: String,
    balance: f64,
}

impl AccountSystem {
    fn new() -> Self {
        AccountSystem {
            accounts: Vec::new(),
            system_balance: 0.0,
        }
    }

    fn get_account(&mut self) -> io::Result<()> {
        let json_content = include_str!("account_data.json");
        let users: Vec<Account> = serde_json::from_str(&json_content)?;
        self.accounts = users.clone();
        Ok(())
    }
}

fn main() {
    let mut account_system = AccountSystem::new();
    account_system.get_account().unwrap();

    println!("Initial State:");
    for account in &account_system.accounts {
        println!("{} {}", account.user, account.balance);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_account() {
        let mut account_system = AccountSystem::new();
        account_system.get_account().unwrap();
        assert_eq!(account_system.accounts.len(), 6);
    }
}
