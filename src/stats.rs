use crate::{data::OutcomeId, model::Run};

pub fn run_count(runs: &Vec<Run>, player_name: &String) -> usize {
    runs.iter()
        .filter(|run| run.players.iter().any(|x| x.player_name == *player_name))
        .count()
}

/// The number of wins that a particular player achieved.
pub fn win_count(runs: &Vec<Run>, player_name: &String) -> usize {
    runs.iter()
        .filter(
            |run| match run.players.iter().find(|x| x.player_name == *player_name) {
                Some(player_run) => player_run.outcome != OutcomeId::Defeat,
                None => false,
            },
        )
        .count()
}

/// The rate from 0 to 1 that which the runs of a particular player are _not_ of [OutcomeId::Defeat].
pub fn win_rate(runs: &Vec<Run>, player_name: &String) -> f64 {
    win_count(runs, player_name) as f64 / run_count(runs, player_name) as f64
}
