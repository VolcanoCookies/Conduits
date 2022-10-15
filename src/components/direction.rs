use std::ops::{Add, Neg, Sub};

use bevy::{math::Vec2, prelude::Component};

#[derive(Component, Copy, Clone)]
pub struct Direction(usize);

impl Into<Vec2> for Direction {
    fn into(self) -> Vec2 {
        match self.0 {
            0 => Vec2::new(0., 1.),
            1 => Vec2::new(1., 1.),
            2 => Vec2::new(1., 0.),
            3 => Vec2::new(1., -1.),
            4 => Vec2::new(0., -1.),
            5 => Vec2::new(-1., -1.),
            6 => Vec2::new(-1., 0.),
            7 => Vec2::new(-1., 1.),
            _ => panic!("Wrong direction {}", self.0),
        }
    }
}

impl Into<usize> for Direction {
    fn into(self) -> usize {
        self.0
    }
}

impl Add<Vec2> for Direction {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        self.vec() + rhs
    }
}

impl Sub<Vec2> for Direction {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Self::Output {
        self.vec() - rhs
    }
}

impl Add<Direction> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Direction) -> Self::Output {
        self + rhs.vec()
    }
}

impl Sub<Direction> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Direction) -> Self::Output {
        self - rhs.vec()
    }
}

impl Neg for Direction {
    type Output = Direction;

    fn neg(self) -> Self::Output {
        Direction((self.0 + 2) % 4)
    }
}

impl Direction {
    pub const North: Direction = Direction(0);
    pub const NorthEast: Direction = Direction(1);
    pub const East: Direction = Direction(2);
    pub const SouthEast: Direction = Direction(3);
    pub const South: Direction = Direction(4);
    pub const SouthWest: Direction = Direction(5);
    pub const West: Direction = Direction(6);
    pub const NorthWest: Direction = Direction(7);

    pub const All: [Direction; 8] = [
        Self::North,
        Self::NorthEast,
        Self::East,
        Self::SouthEast,
        Self::South,
        Self::SouthWest,
        Self::West,
        Self::NorthWest,
    ];

    pub fn vec(self) -> Vec2 {
        self.into()
    }
}
