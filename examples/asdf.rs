extern crate fix;
extern crate typenum;

use fix::aliases::si::{Centi, Deca};

fn main() {
    let one_dollar = Centi::from(100i32);
    let two_dollar = one_dollar + one_dollar;
    let four_dollar = two_dollar * 2;
    println!("{:?}", four_dollar);
    let twenty_dollar = two_dollar * Deca::from(1);
    println!("{:?}", twenty_dollar);

    let cents: Centi<_> = twenty_dollar.convert();
    println!("{:?}", cents);

    let twenty_one_dollar = one_dollar + twenty_dollar.convert();
    println!("{:?}", twenty_one_dollar);

    let wat: Deca<_> = twenty_one_dollar.convert();
    println!("{:?}", wat);

    let mut balance = Centi::from(100i32);
    balance += Centi::from(200);
    balance -= Centi::from(50);
    balance *= 2;

    println!("{:?}", balance);
}
