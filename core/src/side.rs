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

