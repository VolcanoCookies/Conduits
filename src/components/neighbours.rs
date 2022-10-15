use std::ops::{Index, IndexMut};

use bevy::prelude::*;

use crate::unknown_entity;

#[derive(Component, Clone)]
pub struct Neighbours {
    up: Entity,
    right: Entity,
    down: Entity,
    left: Entity,
}

impl Index<usize> for Neighbours {
    type Output = Entity;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.up,
            1 => &self.right,
            2 => &self.down,
            3 => &self.left,
            _ => panic!("Bad index for neighbour {}", index),
        }
    }
}

impl IndexMut<usize> for Neighbours {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.up,
            1 => &mut self.right,
            2 => &mut self.down,
            3 => &mut self.left,
            _ => panic!("Bad index for neighbour {}", index),
        }
    }
}

impl Default for Neighbours {
    fn default() -> Self {
        Self {
            up: unknown_entity(),
            right: unknown_entity(),
            down: unknown_entity(),
            left: unknown_entity(),
        }
    }
}
