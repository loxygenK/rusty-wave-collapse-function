use rand::prelude::*;

use std::marker::PhantomData;

use crate::tile::Tile;

pub enum Side {
    Left,
    Top,
    Bottom,
    Right
}
impl Side {
    pub fn of(self, coord: (usize, usize)) -> (usize, usize) {
        match self {
            Side::Left => (coord.0 - 1, coord.1),
            Side::Top => (coord.0, coord.1 - 1),
            Side::Bottom => (coord.0, coord.1 + 1),
            Side::Right => (coord.0 + 1, coord.1),
        }
    }
}

#[derive(Default, Clone)]
pub struct TilePossibility<Id: Eq + Clone> {
    possible_tiles: Vec<Id>
}

impl<Id: Eq + Clone> TilePossibility<Id> {
    pub fn new(possible_tiles: Vec<Id>) -> Self {
        Self { possible_tiles }
    }

    pub fn collapse(&mut self, tile: Id) {
        self.possible_tiles = vec![tile];
    }

    pub fn collapsed(&self) -> bool {
        self.possible_tiles.len() < 2
    }

    pub fn pick_random<R: Rng>(&self, rng: &mut R) -> Option<&Id> {
        self.possible_tiles.choose(rng)
    }
}

pub struct Field<Id: Eq + Clone, Sides: Eq> {
    tiles: Vec<Vec<TilePossibility<Id>>>,
    width: usize,
    height: usize,
    _phantom: PhantomData<Sides>
}

impl<Id: Eq + Clone, Sides: Eq> Field<Id, Sides> {
    pub fn new(
        tiles: &[&dyn Tile<Identifier = Id, Sides = Sides>],
        width: usize,
        height: usize
    ) -> Self {
        let possible_tiles_id = tiles.iter()
            .map(|tile| tile.identifier())
            .collect::<Vec<_>>();

        Self {
            tiles: vec![vec! [ TilePossibility::new(possible_tiles_id); width ]; height],
            width,
            height,
            _phantom: PhantomData
        }
    }

    pub fn at(&self, x: usize, y: usize) -> Option<&TilePossibility<Id>> {
        self.at_mut(x, y).map(|tile| &*tile)
    }

    pub fn at_mut(&mut self, x: usize, y: usize) -> Option<&mut TilePossibility<Id>> {
        self.tiles
            .get_mut(y)
            .and_then(|row| row.get_mut(x))
    }

    pub fn tiles(&self) -> &[Vec<TilePossibility<Id>>] {
        self.tiles.as_ref()
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn collapse_tiles(&mut self) {
        let mut rng = rand::thread_rng();

        while let Some((x, y)) = self.select_least_possible() {
            let tile = self.at_mut(x, y)
                .expect("Must success, since select_least_possible only picks from valid range");

            let picked = tile
                .pick_random(&mut rng)
                .expect("Must success, since select_least_possible only picks uncollapsed (= non-empty) one");

            tile.collapse(picked.clone());
        }
    }

    fn select_least_possible(&self) -> Option<(usize, usize)> {
        // TODO: Making thing working in gool ol' nested for loop - can I make this more fancy
        // using iterator?
        let mut smallest = usize::MAX;
        let mut coord: Option<(usize, usize)> = None;

        for y in 0..self.height {
            for x in 0..self.width {
                let possibility = self.at(x, y)
                    .expect("Must success, since x, y is looped inside width and height!");

                if possibility.possible_tiles.len() < smallest && possibility.collapsed() {
                    coord = Some((x, y));
                    smallest = possibility.possible_tiles.len();
                }
            }
        }

        coord
    }
}
