pub trait Tile {
    type Identifier: Eq + Clone;
    type Sides: Eq;

    fn identifier(&self) -> Self::Identifier;
    fn connect(
        &self,
        tile: &dyn Tile<Identifier = Self::Identifier, Sides = Self::Sides>
    ) -> Vec<Self::Sides>;
}

pub trait SimpleTile {
    type Identifier: Eq + Clone;
    type Sides: Eq;

    fn identifier(&self) -> Self::Identifier;
    fn acceptable_sides(&self) -> Vec<Self::Sides>;

    fn accept(
        &self,
        _tile: &dyn Tile<Identifier = Self::Identifier, Sides = Self::Sides>
    ) -> bool {
        true
    }
}

impl<T: SimpleTile> Tile for T {
    type Identifier = T::Identifier;
    type Sides = T::Sides;

    fn identifier(&self) -> Self::Identifier {
        SimpleTile::identifier(self)
    }

    fn connect(
        &self,
        tile: &dyn Tile<Identifier = Self::Identifier, Sides = Self::Sides>
    ) -> Vec<T::Sides> {
        if self.accept(tile) {
            self.acceptable_sides()
        } else {
            Vec::new()
        }
    }
}
