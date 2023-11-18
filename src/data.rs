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

impl Item {
    pub fn new(id: ItemId) -> Self {
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
        }
    }
}
