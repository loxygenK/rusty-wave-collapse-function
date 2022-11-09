use std::fmt::Display;

use crate::{field::Field, tile::Tile};

pub fn execute_wfc<'tiles, Id: Eq + Clone + Display>(
    tiles: &'tiles [&'tiles dyn Tile<Identifier = Id>],
    width: usize,
    height: usize,
) -> Field<'tiles, Id> {
    let mut field = Field::new(tiles, width, height);
    field.collapse_tiles();

    field
}
