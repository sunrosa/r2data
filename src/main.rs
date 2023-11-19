use data::OutcomeId;

mod data;
mod model;
mod personal_data;
mod stats;

fn main() {
    let runs = personal_data::personal_runs();
}
