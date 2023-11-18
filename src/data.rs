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
    OnKillEffect,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ItemId {
    SoldiersSyringe,
    TougherTimes,
    MonsterTooth,
}

impl From<ItemId> for Item {
    fn from(id: ItemId) -> Self {
        match id {
            SoldiersSyringe => Item {
                id: SoldiersSyringe,
                name: "Soldier's Syringe".into(),
                rarity: White,
                category: vec![Damage],
            },
            TougherTimes => Item {
                id: TougherTimes,
                name: "Tougher Times".into(),
                rarity: White,
                category: vec![Damage, BrotherBlacklist],
            },
            MonsterTooth => Item {
                id: MonsterTooth,
                name: "Monster Tooth".into(),
                rarity: White,
                category: vec![Healing, OnKillEffect],
            },
        }
    }
}
