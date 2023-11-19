use crate::data::ItemId;

mod data;
mod model;
mod personal_data;

fn main() {
    let runs = personal_data::personal_runs();
}
