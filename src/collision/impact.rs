use bevy::prelude::*;

use crate::Health;

#[derive(Component)]
pub struct Impact;

pub struct ImpactEvent {
    pub radius: f32,
    pub vertices: usize,
    pub color: Handle<ColorMaterial>,
    pub translation: Vec3,
}

pub fn spawn(
    mut commands: Commands,
    mut impact_event: EventReader<ImpactEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for ev in impact_event.iter() {
        commands
            .spawn(Impact)
            .insert(Health(10))
            .insert(ColorMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(shape::Circle {
                        radius: ev.radius,
                        vertices: ev.vertices,
                    }))
                    .into(),
                transform: Transform::from_translation(ev.translation + Vec3::new(0.0, 0.0, 1.0)),
                material: ev.color.clone(),
                ..Default::default()
            });
    }
}

pub fn update(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Health, Option<&Parent>, &mut Transform), With<Impact>>,
) {
    for (entity, mut health, parent, mut transform) in query.iter_mut() {
        health.0 -= 1;
        transform.scale -= 0.1;
        if health.0 == 0 {
            if let Some(parent) = parent {
                commands.entity(parent.get()).remove_children(&[entity]);
            }
        }
    }
}
