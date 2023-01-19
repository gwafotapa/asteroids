use bevy::prelude::*;

use crate::{
    boss::{ColorDamaged, Indestructible},
    fire::Damages,
    Health, Mass, Velocity,
};

pub fn apply<'a, I>(
    parts: I,
    damages1: Option<&Damages>,
    damages2: Option<&Damages>,
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
    let [damages1, damages2] = [
        damages2.map_or_else(|| (mass2.0.sqrt() * dv) as u32 / 2000, |d| d.0),
        damages1.map_or_else(|| (mass1.0.sqrt() * dv) as u32 / 2000, |d| d.0),
    ];

    for ((color_material, maybe_color_damaged, mut health, maybe_indestructible), damages) in
        parts.into_iter().zip([damages1, damages2])
    {
        if maybe_indestructible.is_some() {
            continue;
        }
        health.0 = health.0.saturating_sub(damages);
        if let Some(ColorDamaged(wreck_color)) = maybe_color_damaged {
            let color = &mut materials.get_mut(color_material).unwrap().color;
            if health.0 > 0 {
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
