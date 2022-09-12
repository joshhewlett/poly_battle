///
/// Structs
///
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Rotation {
    Up,
    Right,
    Down,
    Left,
}

///
/// Direction implementation
///
impl Default for Direction {
    fn default() -> Self {
        Direction::Right
    }
}

///
/// Rotation implementation
///
impl Default for Rotation {
    fn default() -> Self {
        Rotation::Up
    }
}
