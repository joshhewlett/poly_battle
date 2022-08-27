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

///
/// Direction implementation
///
impl Default for Direction {
    fn default() -> Self {
        Direction::Right
    }
}
