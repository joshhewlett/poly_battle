#[derive(Debug)]
pub enum PlayerInput {
    KeyDown(Key)
}

#[derive(Debug)]
pub enum Key {
    W,
    A,
    S,
    D
}