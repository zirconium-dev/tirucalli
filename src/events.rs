#[derive(Debug, PartialEq )]
pub enum InputState {
    Pressed,
    Released,
}

impl From<f64> for InputState {
    fn from(value: f64) -> Self {
        match value {
            1.0 => Self::Pressed,
            _ => Self::Released,
        }
    }
}

pub enum InputMapping {
    Up,
    Down,
    Left,
    Right,
    AltUp,
    AltDown,
    AltLeft,
    AltRight,
    North,
    South,
    
}
