use bevy::prelude::Component;

use super::position::Position;

#[derive(Component, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Area {
    pub a: Position,
    pub b: Position,
}

impl Area {
    pub fn new<T: Into<Position>>(a: T, b: T) -> Self {
        let a: Position = a.into();
        let b: Position = b.into();
        Self {
            a: a.min(b),
            b: a.max(b),
        }
    }

    pub fn contains(&self, p: Position) -> bool {
        return self.a.x <= p.x && self.a.y <= p.y && self.b.x >= p.x && self.b.y >= p.y;
    }

    pub fn size(&self) -> Position {
        self.b - self.a
    }

    pub fn grow(&self, p: Position) -> Area {
        let mut a = self.a;
        let mut b = self.b;
        if p.x > 0 {
            b.x += p.x;
        } else {
            a.x -= p.x;
        }
        if p.y > 0 {
            b.y += p.y;
        } else {
            a.y -= p.y
        }
        Area { a, b }
    }

    pub fn clamp(&self, c: Area) -> Area {
        Area {
            a: self.a.max(c.a),
            b: self.b.min(c.b),
        }
    }
}

impl Default for Area {
    fn default() -> Self {
        Self {
            a: Default::default(),
            b: Default::default(),
        }
    }
}
