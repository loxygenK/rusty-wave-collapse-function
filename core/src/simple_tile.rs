use crate::{tile::Tile, side::Side};

pub trait SimpleTile {
    type Identifier: Eq + Clone;

    fn identifier(&self) -> Self::Identifier;
    fn acceptable_sides(&self) -> Vec<Side>;

    fn accept(
        &self,
        _tile: &dyn Tile<Identifier = Self::Identifier>
    ) -> bool {
        true
    }
}

impl<T: SimpleTile> Tile for T {
    type Identifier = T::Identifier;

    fn identifier(&self) -> Self::Identifier {
        SimpleTile::identifier(self)
    }

    fn connect(
        &self,
        tile: &dyn Tile<Identifier = Self::Identifier>,
        side: Side
    ) -> bool {
        self.accept(tile) && self.acceptable_sides().contains(&side)
    }
}
