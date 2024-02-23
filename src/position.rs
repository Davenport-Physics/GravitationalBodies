
use crate::vector2::Vector2;

#[derive(Clone, Copy, Debug)]
pub struct Position2 {
    pub x: f64,
    pub y: f64
}

impl Position2 {

    pub fn from(x: f64, y: f64) -> Position2 {
        Position2 { x, y }
    }

    pub fn vector(&self, other: &Position2) -> Vector2 {

        Vector2 {
            x: other.x - self.x,
            y: other.y - self.y
        }

    }

}