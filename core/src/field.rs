use rand::prelude::*;

use std::marker::PhantomData;

use crate::{tile::Tile, side::{ALL_SIDES, Side}};

#[derive(Default, Clone)]
pub struct TilePossibility<Id: Eq + Clone> {
    possible_tiles: Vec<Id>
}

impl<Id: Eq + Clone> TilePossibility<Id> {
    pub fn new(possible_tiles: Vec<Id>) -> Self {
        Self { possible_tiles }
    }

    pub fn set_possible_tiles(&mut self, tile: Vec<Id>) {
        self.possible_tiles = tile;
    }

    pub fn collapse(&mut self, tile: Id) {
        self.possible_tiles = vec![tile];
    }

    pub fn collapsed(&self) -> bool {
        self.possible_tiles.len() < 2
    }

    pub fn get_collapsed(&self) -> Result<Option<&Id>, ()> {
        if !self.collapsed() {
            return Err(());
        }

        Ok(self.possible_tiles.get(0))
    }

    pub fn pick_random<R: Rng>(&self, rng: &mut R) -> Option<&Id> {
        self.possible_tiles.choose(rng)
    }
}

pub struct TileSet<'tiles, Id: Eq + Clone> {
    tiles: &'tiles [&'tiles dyn Tile<Identifier = Id>],
}

impl<'tiles, Id: Eq + Clone> TileSet<'tiles, Id> {
    pub fn new(tiles: &'tiles [&'tiles dyn Tile<Identifier = Id>]) -> Self {
        Self { tiles }
    }

    pub fn get(&self, id: &Id) -> Option<&&dyn Tile<Identifier = Id>> {
        self.tiles.iter().find(|tile| tile.identifier() == *id)
    }
}


pub struct Field<'tiles, Id: Eq + Clone, Sides: Eq> {
    tile_possibility_map: Vec<Vec<TilePossibility<Id>>>,
    tiles: TileSet<'tiles, Id>,
    width: usize,
    height: usize,
    _phantom: PhantomData<Sides>
}

impl<'tiles, Id: Eq + Clone, Sides: Eq> Field<'tiles, Id, Sides> {
    pub fn new(
        tiles: &'tiles [&'tiles dyn Tile<Identifier = Id>],
        width: usize,
        height: usize
    ) -> Self {
        let possible_tiles_id = tiles.iter()
            .map(|tile| tile.identifier())
            .collect::<Vec<_>>();

        Self {
            tile_possibility_map: vec![vec! [ TilePossibility::new(possible_tiles_id); width ]; height],
            tiles: TileSet::new(tiles),
            width,
            height,
            _phantom: PhantomData
        }
    }

    pub fn at(&self, (x, y): (usize, usize)) -> Option<&TilePossibility<Id>> {
        self.tile_possibility_map
            .get(y)
            .and_then(|row| row.get(x))
    }

    pub fn at_mut(&mut self, (x, y): (usize, usize)) -> Option<&mut TilePossibility<Id>> {
        self.tile_possibility_map
            .get_mut(y)
            .and_then(|row| row.get_mut(x))
    }

    pub fn tile_possibility_map(&self) -> &[Vec<TilePossibility<Id>>] {
        self.tile_possibility_map.as_ref()
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn is_in_field(&self, (x, y): (usize, usize)) -> bool {
        (0..self.width).contains(&x) && (0..self.height).contains(&y)
    }

    pub fn collapse_tiles(&mut self) {
        let mut rng = rand::thread_rng();

        while let Some(coord) = self.select_least_possible() {
            let tile = self.at_mut(coord)
                .expect("Must success, since select_least_possible only picks from valid range");

            let picked = tile
                .pick_random(&mut rng)
                .expect("Must success, since select_least_possible only picks uncollapsed (= non-empty) one");

            tile.collapse(picked.clone());

            for side in ALL_SIDES {
                self.collapse_tile(coord, side);
            }
        }
    }

    fn select_least_possible(&self) -> Option<(usize, usize)> {
        // TODO: Making thing working in gool ol' nested for loop - can I make this more fancy
        // using iterator?
        let mut smallest = usize::MAX;
        let mut coord: Option<(usize, usize)> = None;

        for y in 0..self.height {
            for x in 0..self.width {
                let possibility = self.at((x, y))
                    .expect("Must success, since x, y is looped inside width and height!");

                if possibility.possible_tiles.len() < smallest && possibility.collapsed() {
                    coord = Some((x, y));
                    smallest = possibility.possible_tiles.len();
                }
            }
        }

        coord
    }

    fn collapse_tile(&mut self, base_coord: (usize, usize), side: Side) -> Option<()> {
        let base = self.at(base_coord)?;
        let base_id = base.get_collapsed().expect("Base must be collapsed")?;
        let base_tile = self.tiles.get(base_id)?;
        let neighbor = self.at(side.of(base_coord))?;

        let possible_tiles = neighbor
            .possible_tiles
            .iter()
            .filter(|tile_id| {
                let possible_neighbor_tile = self.tiles.get(tile_id).unwrap();
                let acceptable_sides = base_tile.connect(&**possible_neighbor_tile);

                acceptable_sides.contains(&side)
            })
            .map(Clone::clone)
            .collect::<Vec<_>>();

        self.at_mut(side.of(base_coord))
            .unwrap()
            .set_possible_tiles(possible_tiles);

        Some(())
    }
}
