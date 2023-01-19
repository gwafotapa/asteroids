use bevy::prelude::*;

use crate::{
    asteroid::Asteroid,
    component::Part,
    intercepter::Intercepter,
    map::star::Star,
    wreckage::{Wreckage, WreckageDebris},
};

pub fn count_entities(query: Query<Entity>) {
    println!("entities: {}", query.iter().count());
}

pub fn count_asteroids(query: Query<&Asteroid, Without<Part>>) {
    println!("asteroids: {}", query.iter().count());
}

pub fn count_stars(query: Query<&Star>) {
    println!("stars: {}", query.iter().count());
}

pub fn count_intercepters(query: Query<&Intercepter, Without<Part>>) {
    println!("intercepters: {}", query.iter().count());
}

pub fn count_wreckages(query: Query<&Wreckage>) {
    println!("wreckages: {}", query.iter().count());
}

pub fn count_debris(query: Query<&WreckageDebris>) {
    println!("debris: {}", query.iter().count());
}
