use f4n_wcf_core::{tile::Tile, wfc::execute_wfc};
use f4n_wcf_visualizer::start;
use tiles::{TopTile, LeftTile, BottomTile, RightTile, TileType};

#[macro_use]
extern crate f4n_wcf_core;

mod tiles;

fn main() {
    let tiles: &[&dyn Tile<Identifier = TileType>] = &[
        &TopTile,
        &LeftTile,
        &BottomTile,
        &RightTile,
    ];

    let collapsed = execute_wfc(tiles, 3, 3);
    start(collapsed);
}
