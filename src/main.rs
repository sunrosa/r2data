mod data;
mod model;
mod personal_data;

use chrono::{Duration, TimeZone};
use data::{ItemId::*, *};
use model::*;

fn main() {
    // This takes fucking forever to type in. Please disinfect me.
    let runs = personal_data::personal_runs();

    let latest_run = runs.last().unwrap();

    let sunrosa_latest_run = latest_run
        .players
        .iter()
        .find(|x| x.player_name == "sunrosa")
        .unwrap();

    let sunrosa_latest_loadout = sunrosa_latest_run.loadout.as_ref().unwrap();

    println!("{:?}", sunrosa_latest_loadout);
}
