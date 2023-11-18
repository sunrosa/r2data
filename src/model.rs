#[derive(Debug)]
pub struct Item<'a> {
    pub id: ItemId,
    pub name: &'a str,
    pub rarity: Rarity,
    pub category: Category,
}

impl PartialEq<Item<'_>> for Item<'_> {
    fn eq(&self, other: &Item) -> bool {
        self.id == other.id
    }
}

impl Eq for Item<'_> {}

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
}

#[derive(Debug, PartialEq, Eq)]
pub enum ItemId {
    SoldiersSyringe,
}
