use f4n_wcf_core::{tile::Tile, wfc::execute_wfc};
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

    println!("{}", collapsed);

    // collapsed.to_id_vec().iter().for_each(|row| {
    //     println!("{:#?}", row);
    // });
}
