use bevy::prelude::*;

use crate::{collision::Collider, Health};

pub struct Damages {
    pub location: Vec3,
    pub extent: u32,
}

pub trait Damageable {
    fn damage(&self, health: &mut Health, collider: &mut Collider, damages: Damages) {
        health.0 -= damages.extent as i32;
    }
}
