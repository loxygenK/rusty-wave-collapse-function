use crate::side::Side;

pub trait Tile {
    type Identifier: Eq + Clone;

    fn identifier(&self) -> Self::Identifier;
    fn connect(
        &self,
        tile: &dyn Tile<Identifier = Self::Identifier>,
        side: Side
    ) -> bool;
}

pub struct TileSet<'tiles, Id: Eq + Clone> {
    pub tiles: &'tiles [&'tiles dyn Tile<Identifier = Id>],
}

impl<'tiles, Id: Eq + Clone> TileSet<'tiles, Id> {
    pub fn new(tiles: &'tiles [&'tiles dyn Tile<Identifier = Id>]) -> Self {
        Self { tiles }
    }

    pub fn get(&self, id: &Id) -> Option<&&dyn Tile<Identifier = Id>> {
        self.tiles.iter().find(|tile| tile.identifier() == *id)
    }
}
