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
        match serde_json::from_str::<Vec<Account>>(&json_content) {
            Ok(users) => {
                self.accounts = users;
                Ok(())
            }
            Err(err) => {
                eprintln!("Error deserializing account data: {}", err);
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid account data",
                ))
            }
        }
    }

    fn display_accounts(&self) {
        for account in &self.accounts {
            println!("User: {}, Balance: {}", account.user, account.balance);
        }
        println!("System Balance: {}", self.system_balance);
    }

    // 执行交易
    fn execute_transaction(&mut self, transaction: &Transaction) -> Result<(), &'static str> {
        for transfer in &transaction.transfers {
            let from_account_index = self
                .accounts
                .iter()
                .position(|acc| acc.user == transfer.from);
            let to_account_index = self.accounts.iter().position(|acc| acc.user == transfer.to);

            if let Some(from_index) = from_account_index {
                let total_amount = transfer.amount + transfer.fee;
                if let Some(to_index) = to_account_index {
                    if self.accounts[from_index].balance >= total_amount {
                        self.accounts[from_index].balance -= total_amount;
                        self.accounts[to_index].balance += transfer.amount;
                        self.system_balance += transfer.fee;
                    } else {
                        return Err("Insufficient funds for transfer");
                    }
                } else {
                    return Err("Recipient account not found");
                }
            } else {
                return Err("Sender account not found");
            }
        }

        Ok(())
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_account() {
        let mut account_system = AccountSystem::new();
        account_system.get_account().unwrap();
        assert_eq!(account_system.accounts.len(), 5);
    }

    #[test]
    fn test_execute_transaction() {
        let mut account_system = AccountSystem::new();
        account_system.get_account().unwrap();

        let transaction = Transaction {
            transfers: vec![
                Transfer {
                    from: "A".to_string(),
                    to: "B".to_string(),
                    amount: 0.1,
                    fee: 0.0,
                },
                Transfer {
                    from: "B".to_string(),
                    to: "C".to_string(),
                    amount: 9.0,
                    fee: 1.0,
                },
                Transfer {
                    from: "C".to_string(),
                    to: "E".to_string(),
                    amount: 9.0,
                    fee: 0.0,
                },
            ],
        };

        account_system.execute_transaction(&transaction).unwrap();
        assert_eq!(account_system.accounts[0].balance, 0.0);
        assert_eq!(account_system.accounts[1].balance, 90.1);
        assert_eq!(account_system.accounts[2].balance, 0.0);
        assert_eq!(account_system.accounts[3].balance, 1357.0);
        assert_eq!(account_system.accounts[4].balance, 17.0);
        assert_eq!(account_system.system_balance, 1.0);
    }

    #[test]
    fn test_execute_transaction_insufficient_funds() {
        let mut account_system = AccountSystem::new();
        account_system.get_account().unwrap();

        let transaction = Transaction {
            transfers: vec![Transfer {
                from: "A".to_string(),
                to: "B".to_string(),
                amount: 100.0,
                fee: 0.0,
            }],
        };

        assert_eq!(
            account_system.execute_transaction(&transaction),
            Err("Insufficient funds for transfer")
        );
    }

    #[test]
    fn test_execute_transaction_sender_not_found() {
        let mut account_system = AccountSystem::new();
        account_system.get_account().unwrap();

        let transaction = Transaction {
            transfers: vec![Transfer {
                from: "F".to_string(),
                to: "B".to_string(),
                amount: 100.0,
                fee: 0.0,
            }],
        };

        assert_eq!(
            account_system.execute_transaction(&transaction),
            Err("Sender account not found")
        );
    }

    #[test]
    fn test_execute_transaction_recipient_not_found() {
        let mut account_system = AccountSystem::new();
        account_system.get_account().unwrap();

        let transaction = Transaction {
            transfers: vec![Transfer {
                from: "A".to_string(),
                to: "F".to_string(),
                amount: 100.0,
                fee: 0.0,
            }],
        };

        assert_eq!(
            account_system.execute_transaction(&transaction),
            Err("Recipient account not found")
        );
    }

    #[test]
    fn test_execute_transaction_multiple_errors() {
        let mut account_system = AccountSystem::new();
        account_system.get_account().unwrap();

        let transaction = Transaction {
            transfers: vec![
                Transfer {
                    from: "A".to_string(),
                    to: "B".to_string(),
                    amount: 100.0,
                    fee: 0.0,
                },
                Transfer {
                    from: "B".to_string(),
                    to: "C".to_string(),
                    amount: 100.0,
                    fee: 0.0,
                },
            ],
        };

        assert_eq!(
            account_system.execute_transaction(&transaction),
            Err("Insufficient funds for transfer")
        );
    }
}
