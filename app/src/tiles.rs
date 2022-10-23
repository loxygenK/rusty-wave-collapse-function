use f4n_wcf_core::tiles::{Tile, SimpleTile};
use f4n_wcf_visualizer::renderable_tiles::{RenderableTile, RenderableSides};

#[derive(Eq, PartialEq)]
pub enum TileType {
    TopHorn,
    LeftHorn,
    BottomHorn,
    RightHorn
}

pub struct TopHornTile;
impl SimpleTile for TopHornTile {
    type Identifier = TileType;
    type Sides = RenderableSides;

    fn identifier(&self) -> Self::Identifier {
        TileType::TopHorn
    }

    fn acceptable_sides(&self) -> Vec<Self::Sides> {
        enums!(RenderableSides:{Top, Left, Right})
    }
}
impl RenderableTile for TopHornTile {
    fn get_image_path(&self) -> String {
        "images/top.png".to_string()
    }
}

pub struct LeftHornTile;
impl SimpleTile for LeftHornTile {
    type Identifier = TileType;
    type Sides = RenderableSides;

    fn identifier(&self) -> Self::Identifier {
        TileType::LeftHorn
    }

    fn acceptable_sides(&self) -> Vec<Self::Sides> {
        enums!(RenderableSides:{Top, Bottom, Left})
    }
}
impl RenderableTile for LeftHornTile {
    fn get_image_path(&self) -> String {
        "images/left.png".to_string()
    }
}

pub struct BottomHornTile;
impl SimpleTile for BottomHornTile {
    type Identifier = TileType;
    type Sides = RenderableSides;

    fn identifier(&self) -> Self::Identifier {
        TileType::BottomHorn
    }

    fn acceptable_sides(&self) -> Vec<Self::Sides> {
        enums!(RenderableSides:{Bottom, Left, Right})
    }
}
impl RenderableTile for BottomHornTile {
    fn get_image_path(&self) -> String {
        "images/bottom.png".to_string()
    }
}

pub struct RightHornTile;
impl SimpleTile for RightHornTile {
    type Identifier = TileType;
    type Sides = RenderableSides;

    fn identifier(&self) -> Self::Identifier {
        TileType::RightHorn
    }

    fn acceptable_sides(&self) -> Vec<Self::Sides> {
        enums!(RenderableSides:{Top, Bottom, Right})
    }
}
impl RenderableTile for RightHornTile {
    fn get_image_path(&self) -> String {
        "images/right.png".to_string()
    }
}

