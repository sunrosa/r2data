use crate::{data::OutcomeId, model::Run};

pub fn player_runs<'a>(runs: Vec<&'a Run>, player_name: &String) -> Vec<&'a Run> {
    runs.iter()
        .filter(|run| run.players.iter().any(|x| x.player_name == *player_name))
        .map(|x| *x)
        .collect()
}

/// The number of wins that a particular player achieved.
pub fn player_wins<'a>(runs: Vec<&'a Run>, player_name: &String) -> Vec<&'a Run> {
    runs.iter()
        .filter(
            |run| match run.players.iter().find(|x| x.player_name == *player_name) {
                Some(player_run) => player_run.outcome != OutcomeId::Defeat,
                None => false,
            },
        )
        .map(|x| *x)
        .collect()
}

/// The rate from 0 to 1 that which the runs of a particular player are _not_ of [OutcomeId::Defeat].
pub fn win_rate(runs: Vec<&Run>, player_name: &String) -> f64 {
    let runs_by_player: Vec<&Run> = player_runs(runs, player_name);
    player_wins(runs_by_player.clone(), player_name).len() as f64 / runs_by_player.len() as f64
}
