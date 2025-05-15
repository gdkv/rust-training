fn main() {
    #[derive(Debug)]
    struct Address {
        city: String,
        street: String,
        house: u8,
        apartment: u8,
    }
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
        address: Address
    }

    let address: Address = Address{
        city: String::from("Dubna"),
        street: String::from("Pontekorvo"),
        house: 10,
        apartment: 180,
    };
    let person: Person = Person{
        name: String::from("Dima"),
        age: 35,
        address,
    };

    println!("{:#?}", person);
    println!(
        "{}, ({} y.o.) from {}/{}-{} #{}",
        person.name,
        person.age,
        person.address.city,
        person.address.street,
        person.address.house,
        person.address.apartment
    );
}
