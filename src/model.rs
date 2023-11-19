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
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// The preset difficulty of the run.
    pub difficulty: DifficultyId,
    /// The stats of the players who played in the run.
    pub players: Vec<PlayerRun>,
    /// The statistics of the stages encountered during the run. Stages containing items and enemies, that don't increase the stage number, like Void Fields and the Gilded Coast, are included here. However, the Bazaar in particular is not included here.
    pub stages: Vec<StageStats>,
}

/// The stats for a single run _specific to a single player_. There will be multiple of these for a multiplayer run.
#[derive(Debug)]
pub struct PlayerRun {
    /// The name of the player who played in the run.
    pub player_name: String,
    // TODO: Determine whether this includes time during which only the team-mates are alive.
    /// Time spend alive during the run.
    pub time_alive: chrono::Duration,
    /// Kills dealt by the player during the run.
    pub kills: u32,
    /// Kills dealt by minions during the run.
    pub minion_kills: u32,
    /// Number of player deaths during the run.
    pub deaths: u32,
    /// Damage dealt by the player during the run.
    pub damage_dealt: u64,
    /// Damage dealt by minions (drones, etc.) during the run.
    pub minion_damage_dealt: u64,
    // TODO: Determine whether this includes minion damage. Also determine whether this is accurate at all. I have a hunch it may not be, though I have no evidence to back this up.
    /// (Likely) most damage dealt during one hit.
    pub most_damage_dealt: u64,
    /// Damage taken by the player during the run.
    pub damage_taken: u64,
    /// The highest experience level the player reached during the run.
    pub highest_level: u32,
    /// The total amount of gold collected during the run.
    pub gold_collected: u64,
    /// The number of items picked up during the run.
    pub items_collected: u32,
    /// The number of stages completed during the run.
    pub stages_completed: u32,
    /// The number of items _bought_ during the run.
    pub purchases: u32,
    /// The survivor (referred to as "Class" on the stats screen in-game) played for the run and its loadout.
    pub survivor: SurvivorId,
    /// The survivor's loadout the player used during the run.
    pub loadout: Option<SurvivorLoadout>,
    // TODO: Verify the claim below.
    /// The monster that the player was killed by when the run (or their final life) ended.
    pub killed_by: MonsterId,
    /// The result of the run, including victory, fate unknown, or defeat.
    pub outcome: OutcomeId,
    /// The statistics for the stages the player encountered, in order. This is to be synchronized with the stages as stored in [Run] information.
    pub stages: Vec<PlayerStageStats>,
}

#[derive(Debug)]
pub struct StageStats {
    /// Which stage these stats refer to during a run.
    pub stage_id: EnvironmentId,
    /// The time elapsed at the _end_ of the stage.
    pub clock_at_end: Option<chrono::Duration>,
    /// The enemy's scaling level at the _end_ of the stage.
    pub scaling_at_end: Option<u32>,
    /// The number of mountain shrines hit during the stage.
    pub mountain_shrines_hit: Option<u32>,
}

#[derive(Debug)]
pub struct PlayerStageStats {
    /// The items the player were carrying at the _end_ of the particular stage.
    pub items: Vec<(ItemId, u32)>,
    /// The equipment the player was carrying at the _end_ of the particular stage.
    pub equipment: Option<ItemId>,
    /// Whether or not the player died during the particular stage.
    pub died: bool,
}
