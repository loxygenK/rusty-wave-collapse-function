use f4n_wcf_core::{simple_tile::SimpleTile, side::Side};
use f4n_wcf_visualizer::renderable_tiles::RenderableTile;

#[derive(Clone, Eq, PartialEq)]
pub enum TileType {
    Top,
    Left,
    Bottom,
    Right
}

pub struct TopTile;
impl SimpleTile for TopTile {
    type Identifier = TileType;

    fn identifier(&self) -> Self::Identifier {
        TileType::Top
    }

    fn acceptable_sides(&self) -> Vec<Side> {
        enums!(Side:{Top, Left, Right})
    }
}
impl RenderableTile for TopTile {
    fn get_image_path(&self) -> String {
        "images/top.png".to_string()
    }
}

pub struct LeftTile;
impl SimpleTile for LeftTile {
    type Identifier = TileType;

    fn identifier(&self) -> Self::Identifier {
        TileType::Left
    }

    fn acceptable_sides(&self) -> Vec<Side> {
        enums!(Side:{Top, Bottom, Left})
    }
}
impl RenderableTile for LeftTile {
    fn get_image_path(&self) -> String {
        "images/left.png".to_string()
    }
}

pub struct BottomTile;
impl SimpleTile for BottomTile {
    type Identifier = TileType;

    fn identifier(&self) -> Self::Identifier {
        TileType::Bottom
    }

    fn acceptable_sides(&self) -> Vec<Side> {
        enums!(Side:{Bottom, Left, Right})
    }
}
impl RenderableTile for BottomTile {
    fn get_image_path(&self) -> String {
        "images/bottom.png".to_string()
    }
}

pub struct RightTile;
impl SimpleTile for RightTile {
    type Identifier = TileType;

    fn identifier(&self) -> Self::Identifier {
        TileType::Right
    }

    fn acceptable_sides(&self) -> Vec<Side> {
        enums!(Side:{Top, Bottom, Right})
    }
}
impl RenderableTile for RightTile {
    fn get_image_path(&self) -> String {
        "images/right.png".to_string()
    }
}

