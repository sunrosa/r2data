use crate::data::*;

#[derive(Debug)]
pub struct Item {
    pub id: ItemId,
    pub name: String,
    pub rarity: RarityId,
    pub category: Vec<CategoryId>,
}

impl PartialEq<Item> for Item {
    fn eq(&self, other: &Item) -> bool {
        self.id == other.id
    }
}

impl Eq for Item {}

/// The stats for a run as a whole, including all players, and all stages.
#[derive(Debug)]
pub struct Run {
    /// The date and time that the run was played at.
    timestamp: chrono::DateTime<chrono::Utc>,
    /// The preset difficulty of the run.
    difficulty: DifficultyId,
    /// The stats of the players who played in the run.
    players: Vec<PlayerRun>,
    /// The stages encountered, in order, during the run. Stages that don't increase the stage number, like Void Fields, are included here. However, the Bazaar is not included here.
    stages: Vec<EnvironmentId>,
    /// The time elapsed at the _end_ of each stage. Stages that don't increase the stage number, like Void Fields, are included here. However, the Bazaar is not included here.
    clock_at_stage: Vec<std::time::Duration>,
    /// The enemy's scaling level at the _end_ of each stage. Stages that don't increase the stage number, like Void Fields, are included here. However, the Bazaar is not included here.
    scaling_at_stage: Vec<u32>,
}

/// The stats for a single run _specific to a single player_. There will be multiple of these for a multiplayer run.
#[derive(Debug)]
pub struct PlayerRun {
    /// The name of the player who played in the run.
    player_name: String,
    // TODO: Determine whether this includes time during which only the team-mates are alive.
    /// Time spend alive during the run.
    time_alive: std::time::Duration,
    /// Kills dealt by the player during the run.
    kills: u32,
    /// Kills dealt by minions during the run.
    minion_kills: u32,
    /// Number of player deaths during the run.
    deaths: u32,
    /// Damage dealt by the player during the run.
    damage_dealt: u64,
    /// Damage dealt by minions (drones, etc.) during the run.
    minion_damage_dealt: u64,
    // TODO: Determine whether this includes minion damage. Also determine whether this is accurate at all. I have a hunch it may not be, though I have no evidence to back this up.
    /// (Likely) most damage dealt during one hit.
    most_damage_dealt: u64,
    /// Damage taken by the player during the run.
    damage_taken: u64,
    /// The highest experience level the player reached during the run.
    highest_level: u32,
    /// The total amount of gold collected during the run.
    gold_collected: u64,
    /// The number of items picked up during the run.
    items_collected: u32,
    /// The number of stages completed during the run.
    stages_completed: u32,
    /// The number of items _bought_ during the run.
    purchases: u32,
    /// The survivor (referred to as "Class" on the stats screen in-game) played for the run.
    survivor: SurvivorId,
    // TODO: Verify the claim below.
    /// The monster that the player was killed by when the run (or their final life) ended.
    killed_by: MonsterId,
    /// The number of items held by the player at the end of each stage, and the number of each item that the player had. Stages that don't increase the stage number, like Void Fields, are included here. However, the Bazaar is not included here.
    items: Vec<Vec<(ItemId, u32)>>,
    /// The equipment held by the player at the end of each stage. Stages that don't increase the stage number, like Void Fields, are included here. However, the Bazaar is not included here.
    equipment: Vec<ItemId>,
    /// The result of the run, including victory, fate unknown, or defeat.
    outcome: OutcomeId,
}
