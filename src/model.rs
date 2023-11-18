#[derive(Debug)]
pub struct ItemData<'a> {
    pub name: &'a str,
    pub rarity: Rarity,
    pub category: Category,
}

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
