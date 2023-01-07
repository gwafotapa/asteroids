use bevy::prelude::*;

use crate::{
    boss::{Damaged, Indestructible},
    collision::Collider,
    Health,
};

pub struct Damages {
    pub location: Vec3,
    pub extent: u32,
}

pub trait Damageable {
    fn damage(&self, health: &mut Health, collider: &mut Collider, damages: Damages) {
        health.0 -= damages.extent as i32;
    }
}

pub struct DamageEvent {
    pub entity: Entity,
    // location: Vec3,
    pub extent: u32,
}

pub fn apply(
    mut damage_event: EventReader<DamageEvent>,
    mut query: Query<(
        &Handle<ColorMaterial>,
        Option<&Damaged>,
        &mut Health,
        Option<&Indestructible>,
    )>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for ev in damage_event.iter() {
        let (color, maybe_damaged, mut health, maybe_indestructible) =
            query.get_mut(ev.entity).unwrap();
        if maybe_indestructible.is_some() {
            continue;
        }
        health.0 -= ev.extent as i32;
        if let Some(Damaged(wreck_color)) = maybe_damaged {
            let color = &mut materials.get_mut(color).unwrap().color;
            if health.0 > 0 {
                //     *color = *wreck_color;
                // } else {
                let [wr, wg, wb, _] = wreck_color.as_rgba_f32();
                let [mut r, mut g, mut b, _] = color.as_rgba_f32();
                r += (wr - r) / health.0 as f32;
                g += (wg - g) / health.0 as f32;
                b += (wb - b) / health.0 as f32;
                *color = Color::rgb(r, g, b);
            }
        }
    }
}
