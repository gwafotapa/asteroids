use bevy::prelude::*;

#[derive(Clone, Copy, Debug, Eq)]
pub struct Collision(pub Entity, pub Entity);

impl PartialEq for Collision {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0)
    }
}

#[derive(Debug, Default, Resource)]
pub struct Cache {
    pub old: Vec<Collision>,
    pub new: Vec<Collision>,
}

impl Cache {
    pub fn add(&mut self, collision: Collision) {
        self.new.push(collision);
    }

    pub fn contains(&self, collision: Collision) -> bool {
        self.old.contains(&collision)
    }

    pub fn contains_entity(&self, e: Entity) -> bool {
        self.old.iter().any(|&Collision(a, b)| e == a || e == b)
    }

    pub fn update(&mut self) {
        std::mem::swap(&mut self.old, &mut self.new);
        self.new.clear();
    }
}

pub fn update(mut cache: ResMut<Cache>) {
    cache.update();
}
