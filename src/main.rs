mod account;

fn main() {
    let account_system = account::get_account().unwrap();

    println!("Initial State:");
    for account in &account_system {
        println!("{} {}", account.user, account.balance);
    }
}
