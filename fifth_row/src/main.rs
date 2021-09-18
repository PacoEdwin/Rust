pub mod fifth;
pub use fifth::{parse, Accountant, Error};

use fehler::throws;
use std::io::{self, BufRead};

#[throws]
fn main() {
    let mut accountant = Accountant::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let input = line.unwrap();

        let order = parse(input)?;
        accountant.process_order(order);
    }
}
s