use crate::tile::TileSet;
use rand::prelude::*;

use std::fmt::{Debug, Display};

use crate::{
    side::{Side, ALL_SIDES},
    tile::Tile,
};

#[derive(Default, Clone, PartialEq, Eq)]
pub struct TilePossibility<Id: Eq + Clone> {
    possible_tiles: Vec<Id>,
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

#[derive(PartialEq, Eq)]
pub struct Field<'tiles, Id: Eq + Clone> {
    pub tile_possibility_map: Vec<Vec<TilePossibility<Id>>>,
    pub tiles: TileSet<'tiles, Id>,
    pub width: usize,
    pub height: usize,
}

impl<'tiles, Id: Eq + Clone> Field<'tiles, Id> {
    pub fn new(
        tiles: &'tiles [&'tiles dyn Tile<Identifier = Id>],
        width: usize,
        height: usize,
    ) -> Self {
        let possible_tiles_id = tiles
            .iter()
            .map(|tile| tile.identifier())
            .collect::<Vec<_>>();

        Self {
            tile_possibility_map: vec![
                vec![TilePossibility::new(possible_tiles_id); width];
                height
            ],
            tiles: TileSet::new(tiles),
            width,
            height,
        }
    }

    pub fn at(&self, (x, y): (usize, usize)) -> Option<&TilePossibility<Id>> {
        self.tile_possibility_map.get(y).and_then(|row| row.get(x))
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

    pub fn to_id_vec(&self) -> Vec<Vec<Option<Id>>> {
        let mut id_vec: Vec<Vec<Option<Id>>> = vec![vec![None; self.width]; self.height];

        (0..self.height).for_each(|y| {
            (0..self.width).for_each(|x| {
                id_vec[y][x] = self
                    .at((x, y))
                    .expect("Must success, since x, y counts only inside the valid area")
                    .get_collapsed()
                    .ok()
                    .flatten()
                    .cloned();
            });
        });

        id_vec
    }

    pub fn collapse_tiles(&mut self) {
        let mut rng = rand::thread_rng();

        while let Some(coord) = self.select_least_possible() {
            let tile = self
                .at_mut(coord)
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
                let possibility = self
                    .at((x, y))
                    .expect("Must success, since x, y is looped inside width and height!");

                if possibility.possible_tiles.len() < smallest && !possibility.collapsed() {
                    coord = Some((x, y));
                    smallest = possibility.possible_tiles.len();
                }
            }
        }

        coord
    }

    fn collapse_tile(&mut self, base_coord: (usize, usize), side: Side) -> Option<()> {
        let neighbor = self.at(side.of(base_coord)?)?;
        if neighbor.collapsed() {
            return Some(());
        }

        let possible_tiles = neighbor
            .possible_tiles
            .iter()
            .filter(|tile_id| {
                let possible_neighbor_tile = self.tiles.get(tile_id).unwrap();
                self.is_possible(base_coord, side, *possible_neighbor_tile)
                    .unwrap_or(false)
            })
            .map(Clone::clone)
            .collect::<Vec<_>>();

        self.at_mut(side.of(base_coord)?)
            .unwrap()
            .set_possible_tiles(possible_tiles);

        Some(())
    }

    fn is_possible(
        &self,
        base_coord: (usize, usize),
        side: Side,
        tile: &dyn Tile<Identifier = Id>,
    ) -> Option<bool> {
        let base = self.at(base_coord).expect("Must be in the valid range");
        let base_id = base.get_collapsed().expect("Base must be collapsed")?;
        let base_tile = self.tiles.get(base_id).expect("Must be existing ID");
        let neighbor_coord = side.of(base_coord);

        if neighbor_coord.is_none() {
            return Some(true);
        }

        let base_connectivity = base_tile.connect(tile, side);
        let neighbor_connectivity = tile.connect(*base_tile, side.facing());
        Some(base_connectivity == neighbor_connectivity)
    }

    fn format_as_tile(&self, formatter: impl Fn(&TilePossibility<Id>) -> String) -> String {
        self.tile_possibility_map
            .iter()
            .map(|row| {
                row.iter()
                    .map(|tile| formatter(tile))
                    .collect::<Vec<_>>()
                    .join("")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl<Id: Eq + Clone + Debug> Debug for Field<'_, Id> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max = self
            .tiles
            .tiles
            .iter()
            .map(|m| format!("{:#?}", m.identifier()).chars().count())
            .max()
            .ok_or(std::fmt::Error)?;

        let maximum_length = max * (self.tiles.tiles.len() * 2 - 1);

        let formatted = self.format_as_tile(|tile| {
            let unpadded = tile
                .possible_tiles
                .iter()
                .map(|m| format!("{:#?}", m))
                .collect::<Vec<_>>()
                .join(" ");

            format!(
                "\x1b[38;5;{}m[ {}{} ]{}",
                if tile.collapsed() {
                    "3;1"
                } else if tile.possible_tiles.len() == self.tiles.tiles.len() {
                    "4"
                } else {
                    "7"
                },
                unpadded,
                " ".repeat(maximum_length - unpadded.chars().count()),
                "\x1b[m"
            )
        });
        f.write_str(&formatted)
    }
}

impl<Id: Eq + Clone + Display> Display for Field<'_, Id> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max = self
            .tiles
            .tiles
            .iter()
            .map(|m| format!("{}", m.identifier()).chars().count())
            .max()
            .ok_or(std::fmt::Error)?;

        let formatted = self.format_as_tile(|tile| {
            let unformatted = tile
                .get_collapsed()
                .ok()
                .flatten()
                .map(|m| format!("{}", m))
                .unwrap_or_else(|| "･".to_string());

            format!(
                "{}{}",
                unformatted,
                " ".repeat(max - unformatted.chars().count()),
            )
        });

        f.write_str(&formatted)
    }
}
