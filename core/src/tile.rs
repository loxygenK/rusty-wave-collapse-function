use crate::side::Side;

pub trait Tile {
    type Identifier: Eq + Clone;

    fn identifier(&self) -> Self::Identifier;
    fn connect(&self, tile: &dyn Tile<Identifier = Self::Identifier>, side: Side) -> bool;
}

pub struct TileSet<'tiles, Id: Eq + Clone> {
    pub tiles: &'tiles [&'tiles dyn Tile<Identifier = Id>],
}
impl<'tiles, Id: Eq + Clone> PartialEq for TileSet<'tiles, Id> {
    fn eq(&self, other: &Self) -> bool {
        if self.tiles.len() != other.tiles.len() {
            return false;
        }

        let other_tile_ids = other
            .tiles
            .iter()
            .map(|t| t.identifier())
            .collect::<Vec<_>>();

        self.tiles
            .iter()
            .all(|t| other_tile_ids.contains(&t.identifier()))
    }
}
impl<'tiles, Id: Eq + Clone> Eq for TileSet<'tiles, Id> {}

impl<'tiles, Id: Eq + Clone> TileSet<'tiles, Id> {
    pub fn new(tiles: &'tiles [&'tiles dyn Tile<Identifier = Id>]) -> Self {
        Self { tiles }
    }

    pub fn get(&self, id: &Id) -> Option<&&dyn Tile<Identifier = Id>> {
        self.tiles.iter().find(|tile| tile.identifier() == *id)
    }
}
