use f4n_wcf_core::tile::Tile;

#[derive(Eq, PartialEq)]
pub enum RenderableSides {
    Left,
    Top,
    Right,
    Bottom
}

pub trait RenderableTile: Tile<Sides = RenderableSides> {
    fn get_image_path(&self) -> String;
}
