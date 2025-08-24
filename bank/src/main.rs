#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {id, holder, balance: 0}
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>
}

impl Bank {
    fn new() -> Self {
        Bank {accounts: vec![]}
    }
}

fn add_account(bank: &mut Bank, account: Account) {
    bank.accounts.push(account);
}

fn print_account(account: Account) {
    let bank = Bank::new();
    let account = Account::new(1, String::from("me"));

    println!("{:#?}", account);
}

fn main() {
    let mut bank = Bank::new();
    let account = Account::new(1, String::from("me"));

    add_account(&mut bank, account);
    println!("{:#?}", bank);
}
