use std::ops::Index;

use bevy::prelude::*;

use super::cell_state::CellState;

#[derive(Component)]
pub struct StateMap {
    vec: Vec<CellState>,
    pub width: usize,
    pub height: usize,
}

impl StateMap {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            vec: vec![CellState::Empty; (width * height) as usize],
            width: width as usize,
            height: height as usize,
        }
    }

    pub fn get<T: Into<Vec2>>(&self, p: T) -> CellState {
        let index = self.pos_to_index(p);
        return self.vec.get(index).unwrap().clone();
    }

    pub fn set<T: Into<Vec2>>(&mut self, p: T, e: CellState) {
        let index = self.pos_to_index(p);
        self.vec[index] = e;
    }

    pub fn inside<T: Into<Vec2>>(&self, p: T) -> bool {
        let pos: Vec2 = p.into();
        return pos.x >= 0.
            && pos.x < self.width as f32
            && pos.y >= 0.
            && pos.y < self.height as f32;
    }

    pub fn pos_to_index<T: Into<Vec2>>(&self, p: T) -> usize {
        let pos: Vec2 = p.into();
        return (pos.y * self.width as f32 + pos.x) as usize;
    }

    pub fn index_to_pos(&self, i: usize) -> Vec2 {
        return Vec2 {
            x: (i % self.width) as f32,
            y: (i / self.width) as f32,
        };
    }
}
