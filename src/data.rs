use crate::model::{Category, ItemData, Rarity};

#[derive(Debug)]
pub enum Item<'a> {
    SoldiersSyringe(ItemData<'a>),
}

pub const SoldiersSyringe: Item = Item::SoldiersSyringe(ItemData {
    name: "Soldier's Syringe",
    rarity: Rarity::White,
    category: Category::Damage,
});
