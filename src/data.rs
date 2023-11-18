use crate::model::*;
use {Category::*, ItemId::*, Rarity::*};

#[derive(Debug)]
pub enum Rarity {
    /// Common
    White,
    /// Uncommon
    Green,
    /// Legendary
    Red,
    /// Boss
    Yellow,
    /// Lunar
    Blue,
    /// Void
    Purple,
    /// Equipment
    Orange,
}

#[derive(Debug)]
pub enum Category {
    Damage,
    Healing,
    Utility,
    BrotherBlacklist,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ItemId {
    SoldiersSyringe,
    TougherTimes,
}

pub const SOLDIERS_SYRINGE: Item = Item {
    id: SoldiersSyringe,
    name: "Soldier's Syringe",
    rarity: White,
    category: &[Damage],
};

pub const TOUGHER_TIMES: Item = Item {
    id: TougherTimes,
    name: "Tougher Times",
    rarity: White,
    category: &[Utility, BrotherBlacklist],
};
