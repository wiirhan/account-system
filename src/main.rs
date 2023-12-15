mod account;

use account::{AccountSystem, Transaction, Transfer};

fn main() {
    let mut account_system = AccountSystem::new();
    account_system.get_account().unwrap();

    let transaction = Transaction {
        transfers: vec![Transfer {
            from: "A".to_string(),
            to: "B".to_string(),
            amount: 0.1,
            fee: 0.0,
        }],
    };

    if let Err(e) = account_system.execute_transaction(&transaction) {
        eprintln!("Error executing transaction: {}", e);
    }

    account_system.display_accounts();
}
