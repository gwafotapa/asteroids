use bevy::prelude::*;

use crate::{
    collision::{impact::ImpactEvent, Aabb},
    AngularVelocity, Collider, Health, Mass, MomentOfInertia, Part, Topology, Velocity,
};

#[derive(Component)]
pub struct Fire {
    pub impact_radius: f32,
    pub impact_vertices: usize,
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Damages(pub u32);

pub struct FireEvent {
    pub fire: Fire,
    pub enemy: bool,
    pub damages: u32,
    pub radius: f32,
    pub vertices: usize,
    pub color: Color,
    pub range: f32,
    pub translation: Vec3,
    pub velocity: Velocity,
}

pub fn spawn(
    mut fire_event: EventReader<FireEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for ev in fire_event.iter() {
        let fire = commands
            .spawn(Fire {
                impact_radius: ev.fire.impact_radius,
                impact_vertices: ev.fire.impact_vertices,
            })
            .insert(Damages(ev.damages))
            .insert(Mass(1.0))
            .insert(MomentOfInertia(1.0))
            .insert(ev.velocity)
            .insert(AngularVelocity(0.0))
            .insert(SpatialBundle {
                transform: Transform::from_translation(ev.translation)
                    .with_scale(Vec3::new(ev.range, ev.range, 0.0)),
                ..Default::default()
            })
            .id();

        if ev.enemy {
            commands.entity(fire).insert(Enemy);
        }

        let fire_part = commands
            .spawn(Fire {
                impact_radius: ev.fire.impact_radius,
                impact_vertices: ev.fire.impact_vertices,
            })
            .insert(Part)
            .insert(Health(1))
            .insert(Collider {
                aabb: Aabb { hw: 0.0, hh: 0.0 },
                topology: Topology::Point,
            })
            .insert(ColorMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(shape::Circle {
                        radius: ev.radius,
                        vertices: ev.vertices,
                    }))
                    .into(),
                material: materials.add(ev.color.into()),
                ..Default::default()
            })
            .id();

        commands.entity(fire).add_child(fire_part);
    }
}

pub fn movement(
    mut query: Query<(&mut Transform, &Velocity, &AngularVelocity), With<Fire>>,
    time: Res<Time>,
) {
    for (mut transform, velocity, angular_velocity) in query.iter_mut() {
        transform.translation += velocity.0 * time.delta_seconds();
        transform.rotation *=
            Quat::from_axis_angle(Vec3::Z, angular_velocity.0 * time.delta_seconds());
        transform.scale -= Vec3::new(1.0, 1.0, 0.0);
    }
}

pub fn impact(
    mut impact_event: EventWriter<ImpactEvent>,
    query: Query<(&Children, &Fire, &Transform), Without<Part>>,
    query_part: Query<(&Handle<ColorMaterial>, &Health), With<Fire>>,
) {
    for (children, fire, transform) in query.iter() {
        let (color, health) = query_part.get(children[0]).unwrap();
        if health.0 == 0 {
            impact_event.send(ImpactEvent {
                radius: fire.impact_radius,
                vertices: fire.impact_vertices,
                color: color.clone(),
                translation: transform.translation,
            });
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    query_fire: Query<(&Children, Entity, &Transform), With<Fire>>,
    query_fire_part: Query<&Health, With<Fire>>,
) {
    for (children, fire, transform) in query_fire.iter() {
        let health = query_fire_part.get(children[0]).unwrap();
        if health.0 == 0 || transform.scale == Vec3::ZERO {
            commands.entity(fire).despawn_recursive();
        }
    }
}
