#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Side {
    Left,
    Top,
    Bottom,
    Right,
}
impl Side {
    pub fn of(&self, coord: (usize, usize)) -> Option<(usize, usize)> {
        let maybe_shifted = match self {
            Side::Left => (coord.0.checked_sub(1), Some(coord.1)),
            Side::Top => (Some(coord.0), coord.1.checked_sub(1)),
            Side::Bottom => (Some(coord.0), coord.1.checked_add(1)),
            Side::Right => (coord.0.checked_add(1), Some(coord.1)),
        };

        if let (Some(x), Some(y)) = maybe_shifted {
            Some((x, y))
        } else {
            None
        }
    }

    pub fn facing(&self) -> Side {
        match self {
            Side::Left => Side::Right,
            Side::Top => Side::Bottom,
            Side::Bottom => Side::Top,
            Side::Right => Side::Right,
        }
    }
}

pub const ALL_SIDES: [Side; 4] = [Side::Left, Side::Top, Side::Bottom, Side::Right];
