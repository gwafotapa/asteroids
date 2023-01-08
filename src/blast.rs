use bevy::prelude::*;

use crate::Health;

#[derive(Component)]
pub struct Blast;

pub struct BlastEvent {
    pub radius: f32,
    pub vertices: usize,
    pub color: Color,
    pub translation: Vec3,
}

pub fn spawn(
    mut blast_event: EventReader<BlastEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for ev in blast_event.iter() {
        commands
            .spawn(Blast)
            .insert(Health(1))
            .insert(ColorMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(shape::Circle {
                        radius: ev.radius,
                        vertices: ev.vertices,
                    }))
                    .into(),
                transform: Transform::from_translation(ev.translation + Vec3::new(0.0, 0.0, 1.0)),
                material: materials.add(ev.color.into()),
                ..Default::default()
            });
    }
}

pub fn update(
    // mut commands: Commands,
    // mut query: Query<(Entity, &mut Health, Option<&Parent>), With<Blast>>,
    mut query: Query<&mut Health, With<Blast>>,
) {
    // for (blast, mut health, parent) in query.iter_mut() {
    for mut health in query.iter_mut() {
        health.0 -= 1;
        // if health.0 <= 0 {
        //     if let Some(parent) = parent {
        //         commands.entity(parent.get()).remove_children(&[blast]);
        //     }
        // }
    }
}

// pub fn despawn(mut commands: Commands, query: Query<(Entity, &Health), With<Blast>>) {
//     for (blast, health) in query.iter() {
//         if health.0 <= 0 {
//             commands.entity(blast).despawn();
//         }
//     }
// }
