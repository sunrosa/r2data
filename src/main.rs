use std::io::Write;

use data::{ItemId, OutcomeId};

use crate::example::stage_1_item_averages;

mod data;
mod example;
mod model;
mod personal_data;
mod stat;

fn main() {
    println!("{}", stage_1_item_averages())
}
