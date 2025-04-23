fn main() {
    struct Position(f64, f64, f64);

    let _position = Position(0.2, 0.3, 0.4);

    println!("position: {}, {}, {}", _position.0, _position.1, _position.2);
}
