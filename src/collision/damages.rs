use bevy::prelude::*;

use crate::{
    boss::{ColorDamaged, Indestructible},
    // collision::Collider,
    Health,
    Mass,
    Velocity,
};

// pub struct Damages {
//     pub location: Vec3,
//     pub extent: u32,
// }

// pub trait Damageable {
//     fn damage(&self, health: &mut Health, collider: &mut Collider, damages: Damages) {
//         health.0 -= damages.extent as i32;
//     }
// }

// pub struct DamageEvent {
//     pub entity: Entity,
//     // location: Vec3,
//     pub extent: u32,
// }

pub fn apply<'a, I>(
    parts: I,
    mass1: Mass,
    mass2: Mass,
    velocity1: Velocity,
    velocity2: Velocity,
    normal: Vec2,
    materials: &mut Assets<ColorMaterial>,
) where
    I: IntoIterator<
        Item = (
            &'a Handle<ColorMaterial>,
            Option<&'a ColorDamaged>,
            Mut<'a, Health>,
            Option<&'a Indestructible>,
        ),
    >,
{
    let dv = (velocity1.0 - velocity2.0).truncate().dot(normal).abs();
    let damage1 = (mass2.0 / mass1.0 * dv / 10.0) as u32 + 1;
    let damage2 = (mass1.0 / mass2.0 * dv / 10.0) as u32 + 1;

    for ((color_material, maybe_color_damaged, mut health, maybe_indestructible), damage) in
        parts.into_iter().zip([damage1, damage2])
    {
        if maybe_indestructible.is_some() {
            continue;
        }
        // println!("damages extent: {}", ev.extent);
        health.0 = health.0.saturating_sub(damage);
        if let Some(ColorDamaged(wreck_color)) = maybe_color_damaged {
            let color = &mut materials.get_mut(color_material).unwrap().color;
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
