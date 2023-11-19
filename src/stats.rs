use crate::{data::OutcomeId, model::Run};

/// The rate from 0 to 1 that which the runs of a particular player are _not_ of [OutcomeId::Defeat].
pub fn win_rate(runs: &Vec<Run>, player_name: String) -> f64 {
    let win_count = runs
        .iter()
        .filter(
            |run| match run.players.iter().find(|x| x.player_name == player_name) {
                Some(player_run) => player_run.outcome != OutcomeId::Defeat,
                None => false,
            },
        )
        .count();

    win_count as f64 / runs.len() as f64
}
