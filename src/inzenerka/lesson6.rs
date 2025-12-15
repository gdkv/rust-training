mod content;
use content::config::Config;

fn main() {
    let config = Config::load();

    match config {
        Ok(c) => println!("{:#?}", c.str),
        Err(e) => println!("{}", e),
    }
}