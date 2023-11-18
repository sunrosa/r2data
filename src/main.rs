mod data;
mod model;

use data::*;
use model::*;

fn main() {
    println!("{:?}", Item::new(ItemId::SoldiersSyringe))
}
