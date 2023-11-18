use crate::data::*;

#[derive(Debug)]
pub struct Item {
    pub id: ItemId,
    pub name: String,
    pub rarity: Rarity,
    pub category: Vec<Category>,
}

impl PartialEq<Item> for Item {
    fn eq(&self, other: &Item) -> bool {
        self.id == other.id
    }
}

impl Eq for Item {}
