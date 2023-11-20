use data::OutcomeId;

mod data;
mod model;
mod personal_data;
mod stats;

fn main() {
    let runs = personal_data::personal_runs();
    let runs_ref: Vec<&model::Run> = runs.iter().collect();

    println!(
        "{} / {}",
        stats::player_wins(runs_ref.clone(), &"sunrosa".into()).len(),
        stats::player_runs(runs_ref, &"sunrosa".into()).len()
    )
}
