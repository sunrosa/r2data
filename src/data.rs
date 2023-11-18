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

lazy_static::lazy_static! {
    pub static ref SOLDIERS_SYRINGE: Item = Item {
        id: SoldiersSyringe,
        name: "Soldier's Syringe".into(),
        rarity: White,
        category: Damage,
    };

    pub static ref TOUGHER_TIMES: Item = Item {
        id: TougherTimes,
        name: "Tougher Times".into(),
        rarity: White,
        category: Utility,
    };
}
