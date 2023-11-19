use data::OutcomeId;

use crate::stats::run_count;

mod data;
mod model;
mod personal_data;
mod stats;

fn main() {
    let runs = personal_data::personal_runs();

    println!("{:?}", run_count(&runs, &"Veryveryoriginalname".into()))
}
