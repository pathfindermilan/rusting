#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}
impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }
}

fn print_accounts(accounts: Vec<Account>) {
    println!("{:#?}", accounts);
}
fn main() {
    let bank = Bank::new();

    print_accounts(bank.accounts);
}