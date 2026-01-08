struct SomeStruct {
    data: String,
}

impl SomeStruct {
    // Метод должен заимствовать данные из other, а компилятор, согласно правилам вывода лайфтаймов,
    // считает, что заимствование происходит из self.
    fn borrow_other<'a>(&self, other: &'a str) -> &'a str {
        other
    }
}

fn main() {
    let s = String::new();
    let _ = {
        let some_struct = SomeStruct {
            data: String::new(),
        };

        some_struct.borrow_other(&s)
    };

    // Ссылка borrowed существует, а объект some_struct уже уничтожен.
}