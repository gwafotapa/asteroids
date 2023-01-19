use bevy::prelude::*;

#[derive(Clone, Component, Copy)]
pub struct Part;

#[derive(Clone, Component, Copy)]
pub struct Health(pub u32);

#[derive(Clone, Component, Copy)]
pub struct Mass(pub f32);

#[derive(Clone, Component, Copy)]
pub struct MomentOfInertia(pub f32);

#[derive(Clone, Component, Copy, Debug)]
pub struct Velocity(pub Vec3);

#[derive(Clone, Component, Copy, Debug)]
pub struct AngularVelocity(pub f32); // radians per frame

#[derive(Component)]
pub struct Indestructible;

#[derive(Component)]
pub struct ColorDamaged(pub Color);

#[derive(Component)]
pub struct Attack(pub Vec3);
