mod data;
mod model;

use data::*;
use model::*;

fn main() {
    let syringe = data::SoldiersSyringe;
    let data::Item::SoldiersSyringe(item) = syringe;
    println!("{:?}", item.name);
}
