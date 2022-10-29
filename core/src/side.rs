#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Side {
    Left,
    Top,
    Bottom,
    Right
}
impl Side {
    pub fn of(&self, coord: (usize, usize)) -> (usize, usize) {
        match self {
            Side::Left => (coord.0 - 1, coord.1),
            Side::Top => (coord.0, coord.1 - 1),
            Side::Bottom => (coord.0, coord.1 + 1),
            Side::Right => (coord.0 + 1, coord.1),
        }
    }

    pub fn facing(&self) -> Side {
        match self {
            Side::Left => Side::Right,
            Side::Top => Side::Bottom,
            Side::Bottom => Side::Top,
            Side::Right => Side::Right
        }
    }
}

pub const ALL_SIDES: [Side; 4] = [
    Side::Left,
    Side::Top,
    Side::Bottom,
    Side::Right
];
