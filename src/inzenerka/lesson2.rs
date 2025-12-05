/// Счёт клиента в банке
#[derive(Debug)]
struct BankAccount {
    balance: u32, // может быть unsigned, так как всегда >= 0
}

/// Операции с банковским счётом
enum Operation {
    Deposit(u32),
    Withdraw(u32),
    Nullify,
}
//
// /// Принимает аккаунт, обновляет его, и возвращает обновлённые данные
fn operate(account: &mut BankAccount, operation: Operation) {
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
}

fn transfer(from: &mut BankAccount, receivers: &mut [&mut BankAccount], sum: u32) {
    for receiver in receivers.iter_mut() {
        operate(from, Operation::Withdraw(sum));
        operate(receiver, Operation::Deposit(sum));
    }
}

fn main() {
    let mut account_1 = BankAccount { balance: 1000 };
    let mut account_2 = BankAccount { balance: 10 };
    let mut account_3 = BankAccount { balance: 20 };
    let mut account_4 = BankAccount { balance: 0 };

    operate(&mut account_2, Operation::Nullify);
    transfer(&mut account_1, &mut [&mut account_2, &mut account_3, &mut account_4], 20);

    println!("{:#?}", account_1);
    println!("{:#?}", account_2);
    println!("{:#?}", account_3);
    println!("{:#?}", account_4);
}
