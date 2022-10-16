use std::ops::Sub;

use bevy::{math::Vec2, prelude::Component};
use bevy_ecs_tilemap::tiles::TilePos;

#[derive(Component, Debug, Eq, PartialEq, Copy, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn min(&self, other: Position) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    pub fn max(&self, other: Position) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }

    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Sub<Position> for Position {
    type Output = Position;

    fn sub(self, rhs: Position) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl From<TilePos> for Position {
    fn from(t: TilePos) -> Self {
        Self {
            x: t.x as i32,
            y: t.y as i32,
        }
    }
}

impl From<Position> for TilePos {
    fn from(p: Position) -> Self {
        TilePos::new(p.x as u32, p.y as u32)
    }
}

impl From<Vec2> for Position {
    fn from(p: Vec2) -> Self {
        Position {
            x: p.x as i32,
            y: p.y as i32,
        }
    }
}

impl From<Position> for Vec2 {
    fn from(p: Position) -> Self {
        Vec2::new(p.x as f32, p.y as f32)
    }
}

impl From<(i32, i32)> for Position {
    fn from(t: (i32, i32)) -> Self {
        Position { x: t.0, y: t.1 }
    }
}
