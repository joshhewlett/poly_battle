#[derive(Debug)]
pub enum PlayerInput {
    KeyDown(Key),
}

#[derive(Debug)]
pub enum Key {
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    SpaceBar,
    W,
    A,
    S,
    D,
    I,
    J,
    K,
    L,
}
