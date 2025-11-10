/// Счет клиента в банке
struct BankAccount {
    balance: u32, // может быть unsigned, так как всегда >= 0
}

struct TransferAccounts {
    sender: BankAccount,
    receiver: BankAccount,
}

/// Операции с банковским счетом
enum Operation {
    Deposit(u32),
    Withdraw(u32),
    Nullify,
}

/// Принимает аккаунт, обновляет его и возвращает обновленные данные
fn operate(mut account: BankAccount, operation: Operation) -> BankAccount {
    match operation { // выполняем операцию в соответствии с входными данными
        Operation::Deposit(amount) => {
            account.balance += amount;
        }
        Operation::Withdraw(amount) => {
            account.balance -= amount;
        }
        Operation::Nullify => {
            account.balance = 0;
        }
    }

    account
}

fn transfer(accounts: TransferAccounts, amount: u32) -> TransferAccounts {
    if accounts.sender.balance < amount {
        return accounts;
    }

    TransferAccounts {
        sender: operate(accounts.sender, Operation::Withdraw(amount)),
        receiver: operate(accounts.receiver, Operation::Deposit(amount))
    }
}

fn main() {
    let rich_account = BankAccount { balance: 100 };
    let poor_account = BankAccount { balance: 10 };

    let rich_account = operate(rich_account, Operation::Deposit(100));
    let poor_account = operate(poor_account, Operation::Withdraw(1));


    println!("rich account balance: {}", rich_account.balance);
    println!("poor account balance: {}", poor_account.balance);

    let nullified_account = operate(rich_account, Operation::Nullify);
    println!("nullified account balance: {}", nullified_account.balance);

    let accounts = TransferAccounts { sender: poor_account, receiver: nullified_account };
    let accounts = transfer(accounts, 5);
    println!("rich account balance: {}", accounts.sender.balance);
    println!("poor account balance: {}", accounts.receiver.balance);
}
