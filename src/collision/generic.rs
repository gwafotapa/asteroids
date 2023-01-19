use bevy::prelude::*;

use crate::{
    component::{
        AngularVelocity, ColorDamaged, Health, Indestructible, Mass, MomentOfInertia, Part,
        Velocity,
    },
    fire::Damages,
    transform,
};

use super::{
    damages,
    detection::{self, Collider},
    response,
};

pub fn with<C: Component>(
    mut query_c: Query<
        (
            &mut AngularVelocity,
            &Children,
            Option<&Damages>,
            &Mass,
            &MomentOfInertia,
            &mut Transform,
            &mut Velocity,
        ),
        (With<C>, Without<Part>),
    >,
    query_c_part: Query<(&Collider, Entity, &Transform), (With<C>, With<Part>)>,
    mut query_c_part_mut: Query<
        (
            &Handle<ColorMaterial>,
            Option<&ColorDamaged>,
            &mut Health,
            Option<&Indestructible>,
        ),
        (With<C>, With<Part>),
    >,
    meshes: Res<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
) {
    let mut combinations = query_c.iter_combinations_mut();
    'outer: while let Some(
        [(
            mut angular_velocity1,
            children1,
            maybe_damages1,
            mass1,
            moment_of_inertia1,
            mut transform1,
            mut velocity1,
        ), (
            mut angular_velocity2,
            children2,
            maybe_damages2,
            mass2,
            moment_of_inertia2,
            mut transform2,
            mut velocity2,
        )],
    ) = combinations.fetch_next()
    {
        let mut time_c = time.delta_seconds();
        if let Some((contact, entity1p, entity2p)) = detection::intersection_at(
            &mut transform1,
            &mut transform2,
            &mut time_c,
            *mass1,
            *mass2,
            *moment_of_inertia1,
            *moment_of_inertia2,
            *velocity1,
            *velocity2,
            *angular_velocity1,
            *angular_velocity2,
            &query_c_part,
            &query_c_part,
            children1,
            children2,
            Res::clone(&meshes),
        ) {
            response::compute_velocities(
                &mut velocity1,
                &mut velocity2,
                &mut angular_velocity1,
                &mut angular_velocity2,
                *transform1,
                *transform2,
                *mass1,
                *mass2,
                *moment_of_inertia1,
                *moment_of_inertia2,
                contact,
            );

            damages::apply(
                query_c_part_mut.get_many_mut([entity1p, entity2p]).unwrap(),
                maybe_damages1,
                maybe_damages2,
                *mass1,
                *mass2,
                *velocity1,
                *velocity2,
                contact.normal,
                materials.as_mut(),
            );

            if query_c_part_mut
                .get_component::<Health>(entity1p)
                .unwrap()
                .0
                > 0
            {
                *transform1 = transform::at(
                    time.delta_seconds() - time_c,
                    *transform1,
                    *velocity1,
                    *angular_velocity1,
                );
            }

            if query_c_part_mut
                .get_component::<Health>(entity2p)
                .unwrap()
                .0
                > 0
            {
                *transform2 = transform::at(
                    time.delta_seconds() - time_c,
                    *transform2,
                    *velocity2,
                    *angular_velocity2,
                );
            }

            debug!(
                "translation1_c = {}, translation2_c = {}\n\
		     velocity1 = {}, velocity2 = {}\n",
                transform1.translation, transform2.translation, velocity1.0, velocity2.0,
            );

            continue 'outer;
        }
    }
}

pub fn between<C1: Component, C2: Component>(
    mut query_c1: Query<
        (
            &mut AngularVelocity,
            &Children,
            Option<&Damages>,
            &Mass,
            &MomentOfInertia,
            &mut Transform,
            &mut Velocity,
        ),
        (With<C1>, Without<Part>),
    >,
    mut query_c2: Query<
        (
            &mut AngularVelocity,
            &Children,
            Option<&Damages>,
            &Mass,
            &MomentOfInertia,
            &mut Transform,
            &mut Velocity,
        ),
        (With<C2>, Without<Part>, Without<C1>),
    >,
    query_c1_part: Query<(&Collider, Entity, &Transform), (With<C1>, With<Part>)>,
    mut query_c1_part_mut: Query<
        (
            &Handle<ColorMaterial>,
            Option<&ColorDamaged>,
            &mut Health,
            Option<&Indestructible>,
        ),
        (With<C1>, With<Part>),
    >,
    query_c2_part: Query<(&Collider, Entity, &Transform), (With<C2>, With<Part>, Without<C1>)>,
    mut query_c2_part_mut: Query<
        (
            &Handle<ColorMaterial>,
            Option<&ColorDamaged>,
            &mut Health,
            Option<&Indestructible>,
        ),
        (With<C2>, With<Part>, Without<C1>),
    >,
    meshes: Res<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
) {
    'outer: for (
        mut angular_velocity1,
        children1,
        maybe_damages1,
        mass1,
        moment_of_inertia1,
        mut transform1,
        mut velocity1,
    ) in query_c1.iter_mut()
    {
        for (
            mut angular_velocity2,
            children2,
            maybe_damages2,
            mass2,
            moment_of_inertia2,
            mut transform2,
            mut velocity2,
        ) in query_c2.iter_mut()
        {
            let mut time_c = time.delta_seconds();
            if let Some((contact, entity1p, entity2p)) = detection::intersection_at(
                &mut transform1,
                &mut transform2,
                &mut time_c,
                *mass1,
                *mass2,
                *moment_of_inertia1,
                *moment_of_inertia2,
                *velocity1,
                *velocity2,
                *angular_velocity1,
                *angular_velocity2,
                &query_c1_part,
                &query_c2_part,
                children1,
                children2,
                Res::clone(&meshes),
            ) {
                response::compute_velocities(
                    &mut velocity1,
                    &mut velocity2,
                    &mut angular_velocity1,
                    &mut angular_velocity2,
                    *transform1,
                    *transform2,
                    *mass1,
                    *mass2,
                    *moment_of_inertia1,
                    *moment_of_inertia2,
                    contact,
                );

                damages::apply(
                    [
                        query_c1_part_mut.get_mut(entity1p).unwrap(),
                        query_c2_part_mut.get_mut(entity2p).unwrap(),
                    ],
                    maybe_damages1,
                    maybe_damages2,
                    *mass1,
                    *mass2,
                    *velocity1,
                    *velocity2,
                    contact.normal,
                    materials.as_mut(),
                );

                if query_c1_part_mut
                    .get_component::<Health>(entity1p)
                    .unwrap()
                    .0
                    > 0
                {
                    *transform1 = transform::at(
                        time.delta_seconds() - time_c,
                        *transform1,
                        *velocity1,
                        *angular_velocity1,
                    );
                }

                if query_c2_part_mut
                    .get_component::<Health>(entity2p)
                    .unwrap()
                    .0
                    > 0
                {
                    *transform2 = transform::at(
                        time.delta_seconds() - time_c,
                        *transform2,
                        *velocity2,
                        *angular_velocity2,
                    );
                }

                debug!(
                    "translation1_c = {}, translation2_c = {}\n\
			     velocity1 = {}, velocity2 = {}\n",
                    transform1.translation, transform2.translation, velocity1.0, velocity2.0,
                );

                continue 'outer;
            }
        }
    }
}

pub fn among<C1: Component, C2: Component>(
    mut query: Query<
        (
            &mut AngularVelocity,
            &Children,
            Option<&Damages>,
            &Mass,
            &MomentOfInertia,
            &mut Transform,
            &mut Velocity,
        ),
        (Or<(With<C1>, With<C2>)>, Without<Part>),
    >,
    query_part: Query<(&Collider, Entity, &Transform), (Or<(With<C1>, With<C2>)>, With<Part>)>,
    mut query_part_mut: Query<
        (
            &Handle<ColorMaterial>,
            Option<&ColorDamaged>,
            &mut Health,
            Option<&Indestructible>,
        ),
        (Or<(With<C1>, With<C2>)>, With<Part>),
    >,
    meshes: Res<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
) {
    let mut combinations = query.iter_combinations_mut();
    'outer: while let Some(
        [(
            mut angular_velocity1,
            children1,
            maybe_damages1,
            mass1,
            moment_of_inertia1,
            mut transform1,
            mut velocity1,
        ), (
            mut angular_velocity2,
            children2,
            maybe_damages2,
            mass2,
            moment_of_inertia2,
            mut transform2,
            mut velocity2,
        )],
    ) = combinations.fetch_next()
    {
        let mut time_c = time.delta_seconds();
        if let Some((contact, entity1p, entity2p)) = detection::intersection_at(
            &mut transform1,
            &mut transform2,
            &mut time_c,
            *mass1,
            *mass2,
            *moment_of_inertia1,
            *moment_of_inertia2,
            *velocity1,
            *velocity2,
            *angular_velocity1,
            *angular_velocity2,
            &query_part,
            &query_part,
            children1,
            children2,
            Res::clone(&meshes),
        ) {
            response::compute_velocities(
                &mut velocity1,
                &mut velocity2,
                &mut angular_velocity1,
                &mut angular_velocity2,
                *transform1,
                *transform2,
                *mass1,
                *mass2,
                *moment_of_inertia1,
                *moment_of_inertia2,
                contact,
            );

            damages::apply(
                query_part_mut.get_many_mut([entity1p, entity2p]).unwrap(),
                maybe_damages1,
                maybe_damages2,
                *mass1,
                *mass2,
                *velocity1,
                *velocity2,
                contact.normal,
                materials.as_mut(),
            );

            if query_part_mut.get_component::<Health>(entity1p).unwrap().0 > 0 {
                *transform1 = transform::at(
                    time.delta_seconds() - time_c,
                    *transform1,
                    *velocity1,
                    *angular_velocity1,
                );
            }

            if query_part_mut.get_component::<Health>(entity2p).unwrap().0 > 0 {
                *transform2 = transform::at(
                    time.delta_seconds() - time_c,
                    *transform2,
                    *velocity2,
                    *angular_velocity2,
                );
            }

            debug!(
                "translation1_c = {}, translation2_c = {}\n\
		     velocity1 = {}, velocity2 = {}\n",
                transform1.translation, transform2.translation, velocity1.0, velocity2.0,
            );

            continue 'outer;
        }
    }
}
