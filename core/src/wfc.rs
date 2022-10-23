use crate::tiles::Tile;

#[derive(Default)]
pub struct TilePossibility<Identifier: Eq> {
    possible_tiles: Vec<Identifier>,
    collapsed: bool
}

pub struct Field {
    tiles: Vec<Vec<TilePossibility>>>
}

impl Field {
    pub fn new(tiles: &[&dyn Tile<Identifier = Identifier, Sides = Sides>]) {

    }
}

pub fn do_wfc<Identifier: Eq, Sides: Eq>(
    tiles: &[&dyn Tile<Identifier = Identifier, Sides = Sides>],
    width: usize,
    height: usize
) -> Vec<Vec<Option<Identifier>>> {
    let possibles: Vec<Vec<Option<Identifier>>> = initialize_default_vec(width, height);

    

    todo!()
}

fn initialize_default_vec<T: Default>(width: usize, height: usize) -> Vec<Vec<T>> {
    let mut array: Vec<Vec<T>> = Vec::with_capacity(width);

    for _ in 0..width {
        let mut subarray = Vec::with_capacity(height);
        for _ in 0..height {
            subarray.push(T::default());
        }
        array.push(subarray);
    }

    array
}
