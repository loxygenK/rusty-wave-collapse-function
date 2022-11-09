use f4n_wcf_core::{tile::Tile, wfc::execute_wfc};
use tiles::{BottomTile, LeftTile, RightTile, TileType, TopTile};

#[cfg(feature = "wasm")]
use f4n_wcf_visualizer::start;

#[macro_use]
extern crate f4n_wcf_core;

mod tiles;

fn main() {
    let tiles: &[&dyn Tile<Identifier = TileType>] =
        &[&TopTile, &LeftTile, &BottomTile, &RightTile];

    let collapsed = execute_wfc(tiles, 15, 15);

    #[cfg(feature = "wasm")]
    start(collapsed);

    #[cfg(not(feature = "wasm"))]
    println!("{}", collapsed);
}
