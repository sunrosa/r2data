use crate::{data::ItemId, model, personal_data, stat};

pub fn stage_1_item_averages() -> String {
    let binding = personal_data::personal_runs();
    let all_runs: Vec<&model::Run> = binding.iter().collect();

    let player_name = String::from("sunrosa");
    let player_runs = stat::player_runs(all_runs, &player_name);

    let mut stage_1_items: Vec<(ItemId, f64)> = Vec::new();
    for run in &player_runs {
        stage_1_items.extend(
            run.players
                .iter()
                .find(|x| x.player_name == player_name)
                .unwrap()
                .stages[0]
                .items
                .iter()
                .map(|x| (x.0, x.1 as f64)),
        );
    }

    let mut stage_1_item_averages: Vec<(ItemId, f64)> = Vec::new();
    for item in &stage_1_items {
        stage_1_item_averages.push((item.0, item.1 / player_runs.len() as f64));
    }

    stage_1_item_averages.sort_by(|x, y| y.1.partial_cmp(&x.1).unwrap());

    let mut output = "".to_owned();
    for item in stage_1_item_averages {
        output += &*format!("{:.3}: {:?}\n", item.1, item.0);
    }
    output
}
