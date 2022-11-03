use std::fmt::{Display, Debug, Write};

use f4n_wcf_core::{simple_tile::SimpleTile, side::Side};

#[cfg(feature = "wasm")]
use f4n_wcf_visualizer::renderable_tiles::RenderableTile;

#[derive(Clone, Eq, PartialEq)]
pub enum TileType {
    Top,
    Left,
    Bottom,
    Right
}
impl Display for TileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            TileType::Top => '┴',
            TileType::Left => '┤',
            TileType::Bottom => '┬',
            TileType::Right => '├',
        };

        f.write_char(char)
    }
}
impl Debug for TileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
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
#[cfg(feature = "wasm")]
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
#[cfg(feature = "wasm")]
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
#[cfg(feature = "wasm")]
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
#[cfg(feature = "wasm")]
impl RenderableTile for RightTile {
    fn get_image_path(&self) -> String {
        "images/right.png".to_string()
    }
}
