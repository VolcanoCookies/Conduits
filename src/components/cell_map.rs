use bevy::prelude::*;

use crate::unknown_entity;

#[derive(Component)]
pub struct CellMap {
    vec: Vec<Entity>,
    pub width: usize,
    pub height: usize,
}

impl CellMap {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            vec: vec![unknown_entity(); (width * height) as usize],
            width: width as usize,
            height: height as usize,
        }
    }

    pub fn is_empty<T: Into<Vec2>>(&self, p: T) -> bool {
        return self.get(p) == unknown_entity();
    }

    pub fn get<T: Into<Vec2>>(&self, p: T) -> Entity {
        let index = self.pos_to_index(p);
        let entity = self.vec.get(index);
        return if entity == None {
            unknown_entity()
        } else {
            entity.unwrap().clone()
        };
    }

    pub fn set<T: Into<Vec2>>(&mut self, p: T, e: Entity) {
        let index = self.pos_to_index(p);
        self.vec[index] = e;
    }

    pub fn remove<T: Into<Vec2>>(&mut self, p: T) -> Entity {
        let index = self.pos_to_index(p);
        return std::mem::replace(&mut self.vec[index], unknown_entity());
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
