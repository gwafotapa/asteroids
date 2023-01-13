use bevy::{prelude::*, render::mesh::VertexAttributeValues, sprite::Mesh2dHandle};

use crate::{transform, AngularVelocity, Mass, MomentOfInertia, Velocity};
use triangle::TriangleXY;

pub mod triangle;

pub const EPSILON: f32 = 0.001;

#[derive(Clone, Component)]
pub struct Collider {
    pub aabb: Aabb,
    pub topology: Topology,
}

#[derive(Clone, Copy)]
pub struct Aabb {
    pub hw: f32, // half width
    pub hh: f32, // half height
}

#[derive(Clone)]
pub enum Topology {
    Point,
    Disk { radius: f32 },
    Triangles { mesh_handle: Mesh2dHandle },
}

#[derive(Clone, Copy, Debug)]
pub struct Contact {
    pub point: Vec2,
    pub normal: Vec2,
}

// Determines if point p is in the rectangle of center c, half width hw and half height hh
pub fn point_in_rectangle(p: Vec2, c: Vec2, hw: f32, hh: f32) -> bool {
    p.x >= c.x - hw && p.x <= c.x + hw && p.y >= c.y - hh && p.y <= c.y + hh
}

// Determines if point p is in CCW triangle (abc).
//
// p lies in (abc) iff for all three lines (ab), (bc) and (ca),
// p lies on the same side (either left or right) of each line.
// Equivalently, p lies in (abc) iff (det(pa, pb) >= 0, det(pb, pc) >= 0 and det(pc, pa) >= 0) or
// (det(pa, pb) <= 0, det(pb, pc) <= 0 and det(pc, pa) <= 0).
//
// Since (abc) is CCW, this is equivalent to
// det(pa, pb) >= 0, det(pb, pc) >= 0 and det(pc, pa) >= 0
pub fn point_in_triangle(p: Vec2, t: impl Into<TriangleXY>) -> Option<Contact> {
    let [a, b, c] = t.into().to_array();
    let [pa, pb, pc] = [a - p, b - p, c - p];

    if pa.perp_dot(pb) > 0.0 && pb.perp_dot(pc) > 0.0 && pc.perp_dot(pa) > 0.0 {
        Some(Contact {
            point: p,
            normal: ((a + b + c) / 3.0 - p).normalize(),
        })
    } else {
        None
    }
}

// Determines if point p is in CCW triangle (abc).
//
// We approach the problem differently here with barycentric coordinates.
// Overall this costs us an extra addition compared to the previous method,
// but we're keeping it for bookkeeping purposes.
//
// p lies in (abc) iff there exists (s, t) such that p = a + s(b-a) + t(c-a)
// with 0 <= s <= 1, 0 <= t <= 1 and 0 <= s+t <=1
//
// Remark that checking 0 <= s, 0 <= t and s+t <= 1 is enough.
//
// Solving gives :
// s = det(ap, ac) / det(ab, ac)
// t = det(ab, ap) / det(ab, ac)
//
// Since (abc) is CCW, det(ab, ac) >= 0
// Thus to avoid division, we compute instead:
// s = det(ap, ac)
// t = det(ab, ap)
// and check that 0 <= s, 0 <= t and s+t <= det(ab, ac)
pub fn point_in_triangle_bis(p: Vec2, t: impl Into<TriangleXY>) -> bool {
    let [a, b, c] = t.into().to_array();
    let [ab, ac, ap] = [b - a, c - a, p - a];

    let s = ap.perp_dot(ac);
    if s < 0.0 {
        return false;
    }

    let t = ab.perp_dot(ap);
    if t < 0.0 {
        return false;
    }

    s + t <= ab.perp_dot(ac)
}

pub fn rectangles_intersect(center1: Vec2, aabb1: Aabb, center2: Vec2, aabb2: Aabb) -> bool {
    let intersect_x = (center1.x - center2.x).abs() <= aabb1.hw + aabb2.hw;
    let intersect_y = (center1.y - center2.y).abs() <= aabb1.hh + aabb2.hh;

    intersect_x && intersect_y
}

// Determines if the disk of center c and radius r intersects the line segment [ab].
//
// This happens iff the distance from c to [ab] is less than r.
// https://stackoverflow.com/questions/1073336/circle-line-segment-collision-detection-algorithm
pub fn disk_intersects_line_segment(c: Vec2, r: f32, a: Vec2, b: Vec2) -> Option<Contact> {
    let m = point_of_line_segment_closest_to_point(c, a, b);
    let mc = c - m;
    if mc.length() <= r {
        // println!("b");
        Some(Contact {
            point: m,
            normal: mc.normalize(),
        })
    } else {
        None
    }
}

// https://stackoverflow.com/questions/1073336/circle-line-segment-collision-detection-algorithm
fn point_of_line_segment_closest_to_point(p: Vec2, a: Vec2, b: Vec2) -> Vec2 {
    let [ab, ap] = [b - a, p - a];
    let ah = ap.project_onto(ab);

    // Compute the point m of [ab] closest to the disk center
    // Consider k such that ah = k.ab then
    // if k <= 0 then m = a
    // else if k >= 1 then m = b
    // else m = h
    if ah.x.signum() != ab.x.signum() {
        a
    } else if ah.x.abs() > ab.x.abs() {
        b
    } else {
        a + ah
    }
}

// Determines if the circle of center o and radius r intersects the line segment [mn].
// Returns false if the line segment is inside the circle.
//
// https://stackoverflow.com/questions/1073336/circle-line-segment-collision-detection-algorithm
pub fn circle_intersects_line_segment(o: Vec2, r: f32, m: Vec2, n: Vec2) -> bool {
    let mn = n - m;
    let om = m - o;

    let a = mn.dot(mn);
    let b = 2.0 * om.dot(mn);
    let c = om.dot(om) - r * r;

    let delta = b * b - 4.0 * a * c;
    if delta < 0.0 {
        return false;
    }

    let delta_sqrt = delta.sqrt();
    let t1 = -b - delta_sqrt;
    let t2 = -b + delta_sqrt;

    // We have an intersection for each root of the trinomial in [0,1],
    // i.e. when 0 <= t1/(2a) <= 1 or 0 <= t2/(2a) <= 1
    if a > 0.0 {
        (0.0..2.0 * a).contains(&t1) || (0.0..2.0 * a).contains(&t2)
    } else {
        (2.0 * a..0.0).contains(&t1) || (2.0 * a..0.0).contains(&t2)
    }
}

// Determines if the disk of center o and radius r intersects the CCW triangle abc
pub fn disk_intersects_triangle(o: Vec2, r: f32, t: impl Into<TriangleXY>) -> Option<Contact> {
    let [a, b, c] = t.into().to_array();
    // a.distance(o) < r
    disk_intersects_line_segment(o, r, a, b)
        .or_else(|| disk_intersects_line_segment(o, r, b, c))
        .or_else(|| disk_intersects_line_segment(o, r, c, a))
    // || point_in_triangle(o, [a, b, c])
}

pub fn triangles_intersect(
    t1: impl Into<TriangleXY>,
    t2: impl Into<TriangleXY>,
) -> Option<Contact> {
    let [a1, b1, c1] = t1.into().to_array();
    let [a2, b2, c2] = t2.into().to_array();

    // We only need to test 8 line segments intersections.
    line_segments_intersect(a1, b1 - a1, a2, b2 - a2)
        .or_else(|| line_segments_intersect(a1, b1 - a1, b2, c2 - b2))
        .or_else(|| line_segments_intersect(a1, b1 - a1, c2, a2 - c2))
        .or_else(|| line_segments_intersect(b1, c1 - b1, a2, b2 - a2))
        .or_else(|| line_segments_intersect(b1, c1 - b1, b2, c2 - b2))
        .or_else(|| line_segments_intersect(b1, c1 - b1, c2, a2 - c2))
        .or_else(|| line_segments_intersect(c1, a1 - c1, a2, b2 - a2))
        .or_else(|| line_segments_intersect(c1, a1 - c1, b2, c2 - b2))
}

// Determines if line segments [p, p+r] and [q, q+s] intersect
// without checking for the degenerate overlapping case (and returning false in that case)
// https://stackoverflow.com/questions/563198/how-do-you-detect-where-two-line-segments-intersect
pub fn line_segments_intersect(p: Vec2, r: Vec2, q: Vec2, s: Vec2) -> Option<Contact> {
    let rs = r.perp_dot(s);
    if rs == 0.0 {
        return None;
    }
    // let t = (q - p).perp_dot(s);
    // let u = (q - p).perp_dot(r);

    // if rs > 0.0 {
    //     t > 0.0 && u > 0.0 && t < rs && u < rs
    // } else {
    //     t < 0.0 && u < 0.0 && t > rs && u > rs
    // }
    let t = (q - p).perp_dot(s) / rs;
    let u = (q - p).perp_dot(r) / rs;

    if t > 0.0 && u > 0.0 && t < 1.0 && u < 1.0 {
        let c = p + t * r;
        let [cp, cpr, cq, cqs] = [
            (p - c).length(),
            (p + r - c).length(),
            (q - c).length(),
            (q + s - c).length(),
        ];

        let normal = if cp.min(cpr) < cq.min(cqs) {
            (s.perp_dot(if cp < cpr { p } else { p + r } - q).signum() * Vec2::NEG_Y).rotate(s)
        } else {
            (r.perp_dot(if cq < cqs { q } else { q + s } - p).signum() * Vec2::Y).rotate(r)
        }
        .normalize();

        Some(Contact { point: c, normal })
    } else {
        None
    }
}

fn point_in_transformed_triangles(
    point: Transform,
    triangles_transform: Transform,
    vertices: &[[f32; 3]],
) -> Option<Contact> {
    let mut iter = vertices.chunks_exact(3);
    while let Some(&[a, b, c]) = iter.next() {
        if let Some(contact) = point_in_triangle(
            triangles_transform
                .rotation
                .inverse()
                .mul_vec3(point.translation - triangles_transform.translation)
                .truncate(),
            [a, b, c],
        ) {
            return Some(Contact {
                point: triangles_transform
                    .transform_point(contact.point.extend(0.0))
                    .truncate(),
                normal: triangles_transform
                    .rotation
                    .mul_vec3(contact.normal.extend(0.0))
                    .truncate(),
            });
        }
    }

    None
}

fn transformed_triangles_intersect(
    t1: Transform,
    t2: Transform,
    vertices1: &[[f32; 3]],
    vertices2: &[[f32; 3]],
) -> Option<Contact> {
    let mut iter1 = vertices1.chunks_exact(3);
    while let Some(&[a1, b1, c1]) = iter1.next() {
        // Apply t1 to triangle1
        let [mut a1, mut b1, mut c1] = [
            t1.transform_point(Vec3::from(a1)),
            t1.transform_point(Vec3::from(b1)),
            t1.transform_point(Vec3::from(c1)),
        ];

        // Apply t2 inverse to triangle1.
        // We could apply t2 to triangle2 instead but either
        // we would have to recompute it in each iteration of the nested for loop
        // or we would have to allocate to save the results
        [a1, b1, c1] = [
            t2.rotation.inverse().mul_vec3(a1 - t2.translation),
            t2.rotation.inverse().mul_vec3(b1 - t2.translation),
            t2.rotation.inverse().mul_vec3(c1 - t2.translation),
        ];

        let mut iter2 = vertices2.chunks_exact(3);
        while let Some(&[a2, b2, c2]) = iter2.next() {
            if let Some(contact) = triangles_intersect([a1, b1, c1], [a2, b2, c2]) {
                return Some(Contact {
                    point: t2.transform_point(contact.point.extend(0.0)).truncate(),
                    normal: t2.rotation.mul_vec3(contact.normal.extend(0.0)).truncate(),
                    // normal: (t1.translation - t2.translation).truncate().normalize(),
                });
            }
        }
    }

    None
}

fn disk_intersects_transformed_triangles(
    disk: Transform,
    radius: f32,
    triangles_transform: Transform,
    vertices: &[[f32; 3]],
) -> Option<Contact> {
    let mut iter = vertices.chunks_exact(3);
    while let Some(&[a, b, c]) = iter.next() {
        if let Some(contact) = disk_intersects_triangle(
            triangles_transform
                .rotation
                .inverse()
                .mul_vec3(disk.translation - triangles_transform.translation)
                .truncate(),
            radius,
            [a, b, c],
        ) {
            return Some(Contact {
                point: triangles_transform
                    .transform_point(contact.point.extend(0.0))
                    .truncate(),
                normal: triangles_transform
                    .rotation
                    .mul_vec3(contact.normal.extend(0.0))
                    .truncate(),
            });
        }
    }

    None
}

fn disks_intersect(c1: Vec2, r1: f32, c2: Vec2, r2: f32) -> Option<Contact> {
    if c1.distance(c2) <= r1 + r2 {
        let normal = (c1 - c2).normalize();
        Some(Contact {
            point: c2 + r2 * normal,
            normal,
        })
    } else {
        None
    }
}

pub fn intersection(
    t1: Transform,
    t2: Transform,
    c1: &Collider,
    c2: &Collider,
    meshes: Option<Res<Assets<Mesh>>>,
) -> Option<Contact> {
    if !rectangles_intersect(
        t1.translation.truncate(),
        c1.aabb,
        t2.translation.truncate(),
        c2.aabb,
    ) {
        return None;
    }

    match (t1, t2, &c1.topology, &c2.topology) {
        (_, _, Topology::Point, Topology::Point) => Some(Contact {
            point: t1.translation.truncate(),
            normal: Vec2::ZERO,
        }),
        (point, _, Topology::Point, Topology::Disk { radius })
        | (_, point, Topology::Disk { radius }, Topology::Point) => {
            if t1.translation.distance(t2.translation) < *radius {
                Some(Contact {
                    point: point.translation.truncate(),
                    normal: (t1.translation - t2.translation).truncate().normalize(),
                })
            } else {
                None
            }
        }
        (point, triangles, Topology::Point, Topology::Triangles { mesh_handle })
        | (triangles, point, Topology::Triangles { mesh_handle }, Topology::Point) => {
            if let Some(VertexAttributeValues::Float32x3(vertices)) = meshes
                .unwrap()
                .get(&mesh_handle.0)
                .unwrap()
                .attribute(Mesh::ATTRIBUTE_POSITION)
            {
                let mut maybe_contact = point_in_transformed_triangles(point, triangles, vertices);
                if let Topology::Point = c1.topology {
                    if let Some(contact) = maybe_contact.as_mut() {
                        contact.normal = -contact.normal;
                    }
                }
                maybe_contact
            } else {
                panic!("Cannot access triangle's mesh");
            }
        }
        (_, _, Topology::Disk { radius: radius1 }, Topology::Disk { radius: radius2 }) => {
            disks_intersect(
                t1.translation.truncate(),
                *radius1,
                t2.translation.truncate(),
                *radius2,
            )
        }
        (disk, triangles, Topology::Disk { radius }, Topology::Triangles { mesh_handle })
        | (triangles, disk, Topology::Triangles { mesh_handle }, Topology::Disk { radius }) => {
            if let Some(VertexAttributeValues::Float32x3(vertices)) = meshes
                .unwrap()
                .get(&mesh_handle.0)
                .unwrap()
                .attribute(Mesh::ATTRIBUTE_POSITION)
            {
                let mut maybe_contact =
                    disk_intersects_transformed_triangles(disk, *radius, triangles, vertices);
                if let Topology::Disk { radius: _ } = c2.topology {
                    if let Some(contact) = maybe_contact.as_mut() {
                        contact.normal = -contact.normal;
                    }
                }
                maybe_contact
            } else {
                panic!("Cannot access triangle's mesh");
            }
        }
        (
            _,
            _,
            Topology::Triangles {
                mesh_handle: mesh_handle1,
            },
            Topology::Triangles {
                mesh_handle: mesh_handle2,
            },
        ) => {
            if let Some(VertexAttributeValues::Float32x3(vertices1)) = meshes
                .as_ref()
                .unwrap()
                .get(&mesh_handle1.0)
                .unwrap()
                .attribute(Mesh::ATTRIBUTE_POSITION)
            {
                if let Some(VertexAttributeValues::Float32x3(vertices2)) = meshes
                    .as_ref()
                    .unwrap()
                    .get(&mesh_handle2.0)
                    .unwrap()
                    .attribute(Mesh::ATTRIBUTE_POSITION)
                {
                    transformed_triangles_intersect(t1, t2, vertices1, vertices2)
                } else {
                    panic!("Cannot access triangle's mesh");
                }
            } else {
                panic!("Cannot access triangle's mesh");
            }
        }
    }
}

pub fn intersection_at<'a, I, J>(
    transform1: &mut Transform, // transform of entity 1 at time
    transform2: &mut Transform,
    time: &mut f32, // time at which the transforms are taken
    mass1: Mass,
    mass2: Mass,
    moment_of_inertia1: MomentOfInertia,
    moment_of_inertia2: MomentOfInertia,
    velocity1: Velocity,
    velocity2: Velocity,
    angular_velocity1: AngularVelocity,
    angular_velocity2: AngularVelocity,
    parts1: I,
    parts2: J,
    children1: &Children,
    children2: &Children,
    meshes: Res<Assets<Mesh>>,
) -> Option<(Contact, Entity, Entity)>
where
    I: Copy + IntoIterator<Item = (&'a Collider, Entity, &'a Transform)>,
    J: Copy + IntoIterator<Item = (&'a Collider, Entity, &'a Transform)>,
{
    let mut maybe_collision_c: Option<(Contact, Entity, Entity)> = None;
    'outer: for (collider1p, entity1p, transform1p) in parts1.into_iter() {
        if !children1.contains(&entity1p) {
            continue;
        }
        for (collider2p, entity2p, transform2p) in parts2.into_iter() {
            if !children2.contains(&entity2p) {
                continue;
            }
            if let Some(contact_c) = intersection(
                transform::global_of(*transform1p, *transform1),
                transform::global_of(*transform2p, *transform2),
                collider1p,
                collider2p,
                Some(Res::clone(&meshes)),
            ) {
                maybe_collision_c = Some((contact_c, entity1p, entity2p));
                break 'outer;
            }
        }
    }

    if let Some((contact_c, _, _)) = maybe_collision_c {
        let [mut time_a, mut time_c] = [0.0, *time];
        let [mut transform1_a, mut transform2_a] = [
            transform::at(-time_c, *transform1, velocity1, angular_velocity1),
            transform::at(-time_c, *transform2, velocity2, angular_velocity2),
        ];
        let [mut transform1_c, mut transform2_c] = [*transform1, *transform2];

        let [mut v1, mut v2] = [velocity1, velocity2];
        let [mut w1, mut w2] = [angular_velocity1, angular_velocity2];
        super::response::compute_velocities(
            &mut v1,
            &mut v2,
            &mut w1,
            &mut w2,
            transform1_c,
            transform2_c,
            mass1,
            mass2,
            moment_of_inertia1,
            moment_of_inertia2,
            contact_c,
        );
        debug!(
            "\nCollision detected at time tc\n\
             translation1_a = {}, translation2_a = {}\n\
             translation1_c = {}, translation2_c = {}\n\
	     velocity1_c = {}, velocity2_c = {}\n\
             ta = {}, tc = {}, contact = {:?}",
            transform1_a.translation,
            transform2_a.translation,
            transform1_c.translation,
            transform2_c.translation,
            v1.0,
            v2.0,
            time_a,
            time_c,
            contact_c
        );

        while time_c - time_a > EPSILON {
            let time_b = (time_a + time_c) / 2.0;
            let [transform1_b, transform2_b] = [
                transform::at(time_b - time_a, transform1_a, velocity1, angular_velocity1),
                transform::at(time_b - time_a, transform2_a, velocity2, angular_velocity2),
            ];

            let mut maybe_collision_b: Option<(Contact, Entity, Entity)> = None;
            'outer: for (collider1p, entity1p, transform1p) in parts1.into_iter() {
                if !children1.contains(&entity1p) {
                    continue;
                }
                for (collider2p, entity2p, transform2p) in parts2.into_iter() {
                    if !children2.contains(&entity2p) {
                        continue;
                    }
                    if let Some(contact_b) = intersection(
                        transform::global_of(*transform1p, transform1_b),
                        transform::global_of(*transform2p, transform2_b),
                        collider1p,
                        collider2p,
                        Some(Res::clone(&meshes)),
                    ) {
                        maybe_collision_b = Some((contact_b, entity1p, entity2p));
                        break 'outer;
                    }
                }
            }

            if maybe_collision_b.is_some() {
                maybe_collision_c = maybe_collision_b;
                [transform1_c, transform2_c] = [transform1_b, transform2_b];
                time_c = time_b;
            } else {
                [transform1_a, transform2_a] = [transform1_b, transform2_b];
                time_a = time_b;
            }

            debug!(
                "\nta = {}, tc = {}, contact = {:?}",
                time_a, time_c, contact_c
            );
        }

        [*transform1, *transform2] = [transform1_c, transform2_c];
        *time = time_c;
    }

    maybe_collision_c
}
