/// A button on the playdate
#[repr(u8)]
#[allow(clippy::exhaustive_enums)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Button {
    Left = 1,
    Right = 2,
    Up = 4,
    Down = 8,
    B = 16,
    A = 32,
}
