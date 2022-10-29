use f4n_wcf_core::tile::Tile;

pub trait RenderableTile: Tile {
    fn get_image_path(&self) -> String;
}
