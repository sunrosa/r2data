mod data;
mod model;

use data::*;
use model::*;

fn main() {
    let syringe1 = SOLDIERS_SYRINGE;
    let syringe2 = SOLDIERS_SYRINGE;
    println!("{}", syringe1 == syringe2);
}
