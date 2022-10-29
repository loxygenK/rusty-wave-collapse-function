use crate::side::Side;

pub trait Tile {
    type Identifier: Eq + Clone;

    fn identifier(&self) -> Self::Identifier;
    fn connect(
        &self,
        tile: &dyn Tile<Identifier = Self::Identifier>
    ) -> Vec<Side>;
}
