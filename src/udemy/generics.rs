extern crate num_traits;

use std::io::Error;
use num_traits::{ Float, Num };

fn solve<T: Integer>(a: T, b: T) -> Result<T, Error> {

    // if a <= 0.0 || b <= 0.0 {
    //     return Err(Error::other("Nums a or b should not be negative or zero"));
    // }

    let test: f64 = a.to_f64().unwrap();

    Ok((a.powi(2) + b.powi(2)).sqrt())

}
fn main() -> Result<(), Error> {
    let a: i32 = 3;
    let b: i32 = 4;

    let hypotenuse = solve::<i32>(a, b)?;

    println!("{:?}", hypotenuse);

    Ok(())
}