use crate::data::*;

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
