use std::fmt::{Debug, Display, Write};

use f4n_wcf_core::{side::Side, simple_tile::SimpleTile};

#[cfg(feature = "wasm")]
use f4n_wcf_visualizer::renderable_tiles::RenderableTileId;

#[derive(Clone, Eq, PartialEq)]
pub enum TileType {
    Top,
    Left,
    Bottom,
    Right,
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
#[cfg(feature = "wasm")]
impl RenderableTileId for TileType {
    fn get_image_path(&self) -> String {
        match self {
            TileType::Top => "images/top.png",
            TileType::Left => "images/left.png",
            TileType::Bottom => "images/bottom.png",
            TileType::Right => "images/right.png",
        }
        .to_string()
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
