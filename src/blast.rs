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
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
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

pub fn update(mut query: Query<&mut Health, With<Blast>>) {
    for mut health in query.iter_mut() {
        health.0 -= 1;
    }
}
