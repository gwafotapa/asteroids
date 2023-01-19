use bevy::{prelude::*, render::mesh::PrimitiveTopology, sprite::Mesh2dHandle};
use std::f32::consts::PI;

use crate::{
    blast::BlastEvent,
    collision::{detection::triangle::Triangle, Aabb, Collider, Topology},
    fire::{Fire, FireEvent},
    keyboard::KeyboardBindings,
    AngularVelocity, Health, Mass, MomentOfInertia, Part, Velocity, WINDOW_HEIGHT, WINDOW_WIDTH,
};

pub mod flame;

pub const HEALTH: u32 = 100;
// See assets/spaceship.ggb
pub const TRANSLATION: Vec3 = Vec3 {
    x: WINDOW_WIDTH / 2.0,
    y: WINDOW_HEIGHT / 2.0,
    z: Z,
};
pub const Z: f32 = 500.0;

const S1: Vec3 = Vec3 {
    x: -26.0,
    y: -30.0,
    z: 0.0,
};
const S2: Vec3 = Vec3 {
    x: 34.0,
    y: 0.0,
    z: 0.0,
};
const S3: Vec3 = Vec3 {
    x: -16.0,
    y: 0.0,
    z: 0.0,
};
const S4: Vec3 = Vec3 {
    x: -26.0,
    y: 30.0,
    z: 0.0,
};
const S5: Vec3 = Vec3 {
    x: -36.0,
    y: -20.0,
    z: 0.0,
};
const S6: Vec3 = Vec3 {
    x: 15.0,
    y: 0.0,
    z: 0.0,
};
const S7: Vec3 = Vec3 {
    x: -26.0,
    y: 0.0,
    z: 0.0,
};
const S8: Vec3 = Vec3 {
    x: -36.0,
    y: 20.0,
    z: 0.0,
};
const S9: Vec3 = Vec3 {
    x: 1.0,
    y: 16.0,
    z: 0.0,
};
const S10: Vec3 = Vec3 {
    x: 1.0,
    y: 22.0,
    z: 0.0,
};
const S11: Vec3 = Vec3 {
    x: -11.0,
    y: 22.0,
    z: 0.0,
};
const S12: Vec3 = Vec3 {
    x: -11.0,
    y: -22.0,
    z: 0.0,
};
const S13: Vec3 = Vec3 {
    x: 1.0,
    y: -22.0,
    z: 0.0,
};
const S14: Vec3 = Vec3 {
    x: 1.0,
    y: -16.0,
    z: 0.0,
};
pub const TRIANGLES: [Triangle; 6] = [
    Triangle(S1, S2, S3),
    Triangle(S4, S3, S2),
    Triangle(S5, S6, S7),
    Triangle(S8, S7, S6),
    Triangle(S9, S10, S11),
    Triangle(S12, S13, S14),
];
const AABB: Aabb = Aabb { hw: S2.x, hh: S4.y };

#[derive(Component)]
pub struct Spaceship;

pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let v_pos: Vec<[f32; 3]> = TRIANGLES
        .iter()
        .flat_map(|triangle| triangle.to_array())
        .map(|vertex| vertex.to_array())
        .collect();
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);
    let mesh_handle = meshes.add(mesh);

    const AREA: f32 =
        (S2.x - S4.x) * S4.y + (S6.x - S8.x) * S8.y + (S10.x - S11.x) * (S10.y - S9.y)
            - (S6.x + 13.0) * 12.0;
    const MASS: f32 = AREA;
    const MOMENT_OF_INERTIA: f32 = 0.5 * MASS * AREA / PI;

    let spaceship = commands
        .spawn(Spaceship)
        .insert(Mass(MASS))
        .insert(MomentOfInertia(MOMENT_OF_INERTIA))
        .insert(Velocity(Vec3::ZERO))
        .insert(AngularVelocity(0.0))
        .insert(SpatialBundle {
            transform: Transform::from_translation(TRANSLATION),
            ..Default::default()
        })
        .id();

    const COLOR: Color = Color::BLUE;

    let spaceship_part = commands
        .spawn((Spaceship, Part))
        .insert(Health(HEALTH))
        .insert(Collider {
            aabb: AABB,
            topology: Topology::Triangles {
                mesh_handle: Mesh2dHandle(mesh_handle.clone_weak()),
            },
        })
        .insert(ColorMesh2dBundle {
            mesh: mesh_handle.into(),
            material: materials.add(COLOR.into()),
            ..Default::default()
        })
        .id();

    commands.entity(spaceship).add_child(spaceship_part);
}

pub fn attack(
    mut blast_event: EventWriter<BlastEvent>,
    mut fire_event: EventWriter<FireEvent>,
    keys: Res<Input<KeyCode>>,
    query_spaceship: Query<&Transform, (With<Spaceship>, Without<Part>)>,
    query_bindings: Query<&KeyboardBindings>,
) {
    if !keys.just_pressed(query_bindings.single().fire()) {
        return;
    }

    if let Ok(transform) = query_spaceship.get_single() {
        const BLAST_RADIUS: f32 = 8.0;
        const BLAST_VERTICES: usize = 8;
        const ATTACK_COLOR: Color = Color::YELLOW;
        const ATTACK_SOURCE: Vec3 = S2;

        blast_event.send(BlastEvent {
            radius: BLAST_RADIUS,
            vertices: BLAST_VERTICES,
            color: ATTACK_COLOR,
            translation: transform.translation + transform.rotation * ATTACK_SOURCE,
        });

        const FIRE_IMPACT_RADIUS: f32 = 12.0;
        const FIRE_IMPACT_VERTICES: usize = 16;
        const FIRE_DAMAGES: u32 = 1;
        const FIRE_RADIUS: f32 = 3.0 / FIRE_RANGE as f32;
        const FIRE_VERTICES: usize = 4;
        const FIRE_RANGE: u32 = 20;
        const FIRE_VELOCITY: Vec3 = Vec3 {
            x: 1200.0,
            y: 0.0,
            z: 0.0,
        };

        fire_event.send(FireEvent {
            fire: Fire {
                impact_radius: FIRE_IMPACT_RADIUS,
                impact_vertices: FIRE_IMPACT_VERTICES,
            },
            enemy: false,
            damages: FIRE_DAMAGES,
            radius: FIRE_RADIUS,
            vertices: FIRE_VERTICES,
            color: ATTACK_COLOR,
            range: FIRE_RANGE as f32,
            translation: transform.translation + transform.rotation * ATTACK_SOURCE,
            velocity: Velocity(transform.rotation * FIRE_VELOCITY),
        });
    }
}

pub fn movement(
    mut query_spaceship: Query<
        (&mut AngularVelocity, &mut Transform, &mut Velocity),
        (With<Spaceship>, Without<Part>),
    >,
    keys: Res<Input<KeyCode>>,
    query_bindings: Query<&KeyboardBindings>,
    time: Res<Time>,
) {
    let bindings = query_bindings.single();

    if let Ok((mut s_angular_velocity, mut s_transform, mut s_velocity)) =
        query_spaceship.get_single_mut()
    {
        const ROTATION_SPEED: f32 = 20.0;
        if keys.any_pressed([bindings.rotate_left(), KeyCode::Left]) {
            s_angular_velocity.0 += ROTATION_SPEED * time.delta_seconds();
        } else if keys.any_pressed([bindings.rotate_right(), KeyCode::Right]) {
            s_angular_velocity.0 -= ROTATION_SPEED * time.delta_seconds();
        }

        const ACCELERATION: f32 = 500.0;
        if keys.any_pressed([bindings.accelerate(), KeyCode::Up]) {
            s_velocity.0 += ACCELERATION * time.delta_seconds() * (s_transform.rotation * Vec3::X);
        } else if keys.any_pressed([bindings.decelerate(), KeyCode::Down]) {
            s_velocity.0 +=
                0.5 * ACCELERATION * time.delta_seconds() * (s_transform.rotation * Vec3::NEG_X);
        }

        const DRAG: f32 = 0.01;
        s_velocity.0 *= 1.0 - DRAG;
        const ANGULAR_DRAG: f32 = 0.1;
        s_angular_velocity.0 *= 1.0 - ANGULAR_DRAG;

        s_transform.translation += s_velocity.0 * time.delta_seconds();
        s_transform.rotation *=
            Quat::from_axis_angle(Vec3::Z, s_angular_velocity.0 * time.delta_seconds());
    }
}
