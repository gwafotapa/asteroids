use bevy::{prelude::*, render::mesh::VertexAttributeValues, sprite::Mesh2dHandle};
use triangle::TriangleXY;

pub mod triangle;

#[derive(Clone, Component)]
pub struct Collider {
    pub sleep: u32,
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
pub fn point_in_triangle(p: Vec2, t: impl Into<TriangleXY>) -> bool {
    let [a, b, c] = t.into().to_array();
    let [pa, pb, pc] = [a - p, b - p, c - p];

    pa.perp_dot(pb) > 0.0 && pb.perp_dot(pc) > 0.0 && pc.perp_dot(pa) > 0.0
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
pub fn disk_intersects_line_segment(c: Vec2, r: f32, a: Vec2, b: Vec2) -> bool {
    let [ab, ac] = [b - a, c - a];
    let ah = ac.project_onto(ab);

    // Compute the point m of [ab] closest to the disk
    // Consider k such that ah = k.ab then
    // if k <= 0 then m = a
    // else if k >= 1 then m = b
    // else m = h
    let m = if ah.x.signum() != ab.x.signum() {
        a
    } else if ah.x.abs() >= ab.x.abs() {
        b
    } else {
        a + ah
    };

    (m - c).length() < r
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
pub fn disk_intersects_triangle(o: Vec2, r: f32, t: impl Into<TriangleXY>) -> bool {
    let [a, b, c] = t.into().to_array();
    a.distance(o) < r
        || disk_intersects_line_segment(o, r, a, b)
        || disk_intersects_line_segment(o, r, b, c)
        || disk_intersects_line_segment(o, r, c, a)
        || point_in_triangle(o, [a, b, c])
}

pub fn triangles_intersect(t1: impl Into<TriangleXY>, t2: impl Into<TriangleXY>) -> bool {
    let [a1, b1, c1] = t1.into().to_array();
    let [a2, b2, c2] = t2.into().to_array();

    // We only need to test 8 line segments intersections.
    line_segments_intersect(a1, b1 - a1, a2, b2 - a2)
        || line_segments_intersect(a1, b1 - a1, b2, c2 - b2)
        || line_segments_intersect(a1, b1 - a1, c2, a2 - c2)
        || line_segments_intersect(b1, c1 - b1, a2, b2 - a2)
        || line_segments_intersect(b1, c1 - b1, b2, c2 - b2)
        || line_segments_intersect(b1, c1 - b1, c2, a2 - c2)
        || line_segments_intersect(c1, a1 - c1, a2, b2 - a2)
        || line_segments_intersect(c1, a1 - c1, b2, c2 - b2)
}

// Determines if line segments [p, p+r] and [q, q+s] intersect
// without checking for the degenerate overlapping case (and returning false in that case)
// https://stackoverflow.com/questions/563198/how-do-you-detect-where-two-line-segments-intersect
pub fn line_segments_intersect(p: Vec2, r: Vec2, q: Vec2, s: Vec2) -> bool {
    let rs = r.perp_dot(s);
    if rs == 0.0 {
        return false;
    }
    let t = (q - p).perp_dot(s);
    let u = (q - p).perp_dot(r);

    if rs > 0.0 {
        t > 0.0 && u > 0.0 && t < rs && u < rs
    } else {
        t < 0.0 && u < 0.0 && t > rs && u > rs
    }
}

fn point_in_transformed_triangles(
    point: Transform,
    triangles_transform: Transform,
    vertices: &[[f32; 3]],
) -> bool {
    let mut iter = vertices.chunks_exact(3);
    while let Some(&[a, b, c]) = iter.next() {
        if point_in_triangle(
            triangles_transform
                .rotation
                .inverse()
                .mul_vec3(point.translation - triangles_transform.translation)
                .truncate(),
            [a, b, c],
        ) {
            return true;
        }
    }

    false
}

fn transformed_triangles_intersect(
    t1: Transform,
    t2: Transform,
    vertices1: &[[f32; 3]],
    vertices2: &[[f32; 3]],
) -> bool {
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
            if triangles_intersect([a1, b1, c1], [a2, b2, c2]) {
                return true;
            }
        }
    }

    false
}

fn disk_intersects_transformed_triangles(
    disk: Transform,
    radius: f32,
    triangles_transform: Transform,
    vertices: &[[f32; 3]],
) -> bool {
    let mut iter = vertices.chunks_exact(3);
    while let Some(&[a, b, c]) = iter.next() {
        if disk_intersects_triangle(
            triangles_transform
                .rotation
                .inverse()
                .mul_vec3(disk.translation - triangles_transform.translation)
                .truncate(),
            radius,
            [a, b, c],
        ) {
            return true;
        }
    }

    false
}

pub fn collision(
    t1: Transform,
    t2: Transform,
    c1: &Collider,
    c2: &Collider,
    meshes: Option<&Assets<Mesh>>,
) -> bool {
    if !rectangles_intersect(
        t1.translation.truncate(),
        c1.aabb,
        t2.translation.truncate(),
        c2.aabb,
    ) {
        return false;
    }

    match (t1, t2, &c1.topology, &c2.topology) {
        (_, _, Topology::Point, Topology::Point) => true,
        (_, _, Topology::Point, Topology::Disk { radius })
        | (_, _, Topology::Disk { radius }, Topology::Point) => {
            t1.translation.distance(t2.translation) < *radius
        }
        (point, triangles, Topology::Point, Topology::Triangles { mesh_handle })
        | (triangles, point, Topology::Triangles { mesh_handle }, Topology::Point) => {
            if let Some(VertexAttributeValues::Float32x3(vertices)) = meshes
                .unwrap()
                .get(&mesh_handle.0)
                .unwrap()
                .attribute(Mesh::ATTRIBUTE_POSITION)
            {
                point_in_transformed_triangles(point, triangles, vertices)
            } else {
                panic!("Cannot access triangle's mesh");
            }
        }
        (_, _, Topology::Disk { radius: radius1 }, Topology::Disk { radius: radius2 }) => {
            t1.translation.distance(t2.translation) < radius1 + radius2
        }
        (disk, triangles, Topology::Disk { radius }, Topology::Triangles { mesh_handle })
        | (triangles, disk, Topology::Triangles { mesh_handle }, Topology::Disk { radius }) => {
            if let Some(VertexAttributeValues::Float32x3(vertices)) = meshes
                .unwrap()
                .get(&mesh_handle.0)
                .unwrap()
                .attribute(Mesh::ATTRIBUTE_POSITION)
            {
                disk_intersects_transformed_triangles(disk, *radius, triangles, vertices)
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
                .unwrap()
                .get(&mesh_handle1.0)
                .unwrap()
                .attribute(Mesh::ATTRIBUTE_POSITION)
            {
                if let Some(VertexAttributeValues::Float32x3(vertices2)) = meshes
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
