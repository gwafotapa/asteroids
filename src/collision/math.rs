// Triangle devrait etre constitue de Vec3 etant donne que ce code est base sur bevy
// Les modifications doivent etre apportees aux fonctions en consequences :
// Attention aux calculs de normes, produits scalaires et vectoriels
// Ou alors Triangle est constitue de Vec2 et implemente le trait From<Vec3> ?
// Dans le premier cas, faut-il suffixer les fonctions "_2d" ? ou le fichier math2d.rs ?

use bevy::{prelude::*, render::mesh::VertexAttributeValues, sprite::Mesh2dHandle};

#[derive(Clone, Copy)]
pub struct Triangle(pub Vec3, pub Vec3, pub Vec3);

impl Triangle {
    pub fn to_array(&self) -> [Vec3; 3] {
        [self.0, self.1, self.2]
    }
    //     // fn new(a: Vec3, b: Vec3, c: Vec3) -> Triangle {
    //     //     Triangle(a, b, c)
    //     // }

    fn xy(&self) -> TriangleXY {
        TriangleXY(self.0.truncate(), self.1.truncate(), self.2.truncate())
    }
}

// impl From<[Vec3; 3]> for Triangle {
//     fn from(array: [Vec3; 3]) -> Triangle {
//         Triangle(array[0], array[1], array[2])
//     }
// }

// impl From<&[Vec3; 3]> for Triangle {
//     fn from(array: &[Vec3; 3]) -> Triangle {
//         Triangle(array[0], array[1], array[2])
//     }
// }

// impl From<[[f32; 3]; 3]> for Triangle {
//     fn from(array: [[f32; 3]; 3]) -> Triangle {
//         Triangle(
//             Vec3::from(array[0]),
//             Vec3::from(array[1]),
//             Vec3::from(array[2]),
//         )
//     }
// }

// impl From<&[[f32; 3]; 3]> for Triangle {
//     fn from(array: &[[f32; 3]; 3]) -> Triangle {
//         Triangle(
//             Vec3::from(array[0]),
//             Vec3::from(array[1]),
//             Vec3::from(array[2]),
//         )
//     }
// }

#[derive(Clone, Copy)]
pub struct TriangleXY(Vec2, Vec2, Vec2);

impl TriangleXY {
    fn to_array(&self) -> [Vec2; 3] {
        [self.0, self.1, self.2]
    }
}

impl From<[Vec2; 3]> for TriangleXY {
    fn from(array: [Vec2; 3]) -> TriangleXY {
        TriangleXY(array[0], array[1], array[2])
    }
}

impl From<[[f32; 2]; 3]> for TriangleXY {
    fn from(array: [[f32; 2]; 3]) -> TriangleXY {
        TriangleXY(
            Vec2::from(array[0]),
            Vec2::from(array[1]),
            Vec2::from(array[2]),
        )
    }
}

impl From<[Vec3; 3]> for TriangleXY {
    fn from(array: [Vec3; 3]) -> TriangleXY {
        TriangleXY(
            array[0].truncate(),
            array[1].truncate(),
            array[2].truncate(),
        )
    }
}

impl From<[[f32; 3]; 3]> for TriangleXY {
    fn from(array: [[f32; 3]; 3]) -> TriangleXY {
        TriangleXY(
            Vec3::from(array[0]).truncate(),
            Vec3::from(array[1]).truncate(),
            Vec3::from(array[2]).truncate(),
        )
    }
}

impl From<Triangle> for TriangleXY {
    fn from(triangle: Triangle) -> TriangleXY {
        triangle.xy()
    }
}

// trait Triangle2D {
//     fn a(&self) -> Vec2;
//     fn b(&self) -> Vec2;
//     fn c(&self) -> Vec2;
//     fn abc(&self) -> [Vec2; 3] {
//         [self.a(), self.b(), self.c()]
//     }
//     fn triangle2D(&self) -> TriangleXY {
//         TriangleXY::from(self.abc())
//     }
// }

// impl Triangle2D for TriangleXY {
//     fn a(&self) -> Vec2 {
//         self.0
//     }

//     fn b(&self) -> Vec2 {
//         self.1
//     }

//     fn c(&self) -> Vec2 {
//         self.2
//     }
// }

// impl Triangle2D for Triangle {
//     fn a(&self) -> Vec2 {
//         self.0.truncate()
//     }

//     fn b(&self) -> Vec2 {
//         self.1.truncate()
//     }

//     fn c(&self) -> Vec2 {
//         self.2.truncate()
//     }
// }

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
    Circle { radius: f32 },
    Triangles { mesh_handle: Mesh2dHandle },
}

// Determines if point p is in the rectangle of center c, half width x and half height y
pub fn point_in_rectangle(p: Vec2, c: Vec2, x: f32, y: f32) -> bool {
    p.x >= c.x - x && p.x <= c.x + x && p.y >= c.y - y && p.y <= c.y + y
}

pub fn point_in_triangle(p: Vec2, t: impl Into<TriangleXY>) -> bool {
    let t = t.into();
    let denominator = (t.1.y - t.2.y) * (t.0.x - t.2.x) + (t.2.x - t.1.x) * (t.0.y - t.2.y);
    let a = ((t.1.y - t.2.y) * (p.x - t.2.x) + (t.2.x - t.1.x) * (p.y - t.2.y)) / denominator;
    let b = ((t.2.y - t.0.y) * (p.x - t.2.x) + (t.0.x - t.2.x) * (p.y - t.2.y)) / denominator;
    let c = 1.0 - a - b;

    a >= 0.0 && a <= 1.0 && b >= 0.0 && b <= 1.0 && c >= 0.0 && c <= 1.0
}

pub fn rectangles_intersect(position1: Vec2, aabb1: Aabb, position2: Vec2, aabb2: Aabb) -> bool {
    let intersect_x = (position1.x - position2.x).abs() <= aabb1.hw + aabb2.hw;
    let intersect_y = (position1.y - position2.y).abs() <= aabb1.hh + aabb2.hh;

    return intersect_x && intersect_y;
}

// Determines if the circle of center o and radius r intersects the line segment [mn].
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

    let t1 = (-b - delta.sqrt()) / (2.0 * a);
    let t2 = (-b + delta.sqrt()) / (2.0 * a);

    if t1 >= 0.0 && t1 <= 1.0 {
        // t1 is the intersection.
        // Moreover, if t2 is another intersection, t1 is closer to point m than t2 since t1 < t2.
        // Geometrically, line segment [mn] either impales or pokes the circle.
        return true;
    }

    // Here t1 didn't intersect so we are either started inside the circle or completely past it
    if t2 >= 0.0 && t2 <= 1.0 {
        // Geometrically, this is the called the "exit wound" case.
        return true;
    }

    // No intersection.
    // Line segment falls short or is past the circle or is completely inside.
    false
}

// Determines if the disk of center o and radius r intersects the triangle abc
pub fn circle_intersects_triangle(o: Vec2, r: f32, t: impl Into<TriangleXY>) -> bool {
    let triangle = t.into();
    let [a, b, c] = triangle.to_array();
    a.distance(o) < r
        || circle_intersects_line_segment(o, r, a, b)
        || circle_intersects_line_segment(o, r, b, c)
        || circle_intersects_line_segment(o, r, c, a)
        || point_in_triangle(o, triangle)
}

// pub fn collision_circle_triangles(
//     circle_transform: &Transform,
//     radius: f32,
//     circle_aabb: Aabb,
//     triangles_transform: &Transform,
//     vertices: &Vec<[f32; 3]>,
//     triangles_aabb: Aabb,
// ) -> bool {
//     if !rectangles_intersect(
//         circle_transform.translation.truncate(),
//         circle_aabb,
//         triangles_transform.translation.truncate(),
//         triangles_aabb,
//     ) {
//         return false;
//     }

//     for triangle in vertices.chunks_exact(3) {
//         if circle_intersects_triangle(
//             triangles_transform
//                 .rotation
//                 .inverse()
//                 .mul_vec3(circle_transform.translation - triangles_transform.translation)
//                 .truncate(),
//             radius,
//             Vec3::from(triangle[0]).truncate(),
//             Vec3::from(triangle[1]).truncate(),
//             Vec3::from(triangle[2]).truncate(),
//         ) {
//             return true;
//         }
//     }

//     false
// }

// pub fn collision_point_circle(point: &Transform, circle: &Transform, radius: f32) -> bool {
//     if !point_in_rectangle(
//         point.translation.truncate(),
//         circle.translation.truncate(),
//         radius,
//         radius,
//     ) {
//         return false;
//     }

//     point.translation.distance(circle.translation) < radius
// }

// pub fn collision_point_triangles(
//     point: &Transform,
//     triangles: &Transform,
//     vertices: &Vec<[f32; 3]>,
//     aabb: Aabb,
// ) -> bool {
//     if !point_in_rectangle(
//         point.translation.truncate(),
//         triangles.translation.truncate(),
//         aabb.hw,
//         aabb.hh,
//     ) {
//         return false;
//     }

//     for triangle in vertices.chunks_exact(3) {
//         if point_in_triangle(
//             triangles
//                 .rotation
//                 .inverse()
//                 .mul_vec3(point.translation - triangles.translation)
//                 .truncate(),
//             Vec3::from(triangle[0]).truncate(),
//             Vec3::from(triangle[1]).truncate(),
//             Vec3::from(triangle[2]).truncate(),
//         ) {
//             return true;
//         }
//     }

//     false
// }

// Determines if any of the transformed triangles given by vertices1 and transform1 intersects
// any of the transformed triangles given by vertices2 and transform2.
// aabb1 is an aabb centered at transform1.translation and containing all the transformed
// triangles given by vertices1 and transform1.
// Same for aabb2 with respect to transform2 and vertices2.
// fn collision_triangles_triangles(
//     transform1: &Transform,
//     vertices1: &Vec<[f32; 3]>,
//     aabb1: Aabb,
//     transform2: &Transform,
//     vertices2: &Vec<[f32; 3]>,
//     aabb2: Aabb,
// ) -> bool {
//     if !rectangles_intersect(
//         transform1.translation.truncate(),
//         aabb1,
//         transform2.translation.truncate(),
//         aabb2,
//     ) {
//         return false;
//     }

//     let mut iter1 = vertices1.chunks_exact(3);
//     while let Some(&[a1, b1, c1]) = iter1.next() {
//         // Apply transform1 to triangle1
//         let [a1, b1, c1] = [
//             transform1.transform_point(Vec3::from(a1)),
//             transform1.transform_point(Vec3::from(b1)),
//             transform1.transform_point(Vec3::from(c1)),
//         ];

//         // Apply transform2 inverse to triangle1.
//         // We could apply transform2 to triangle2 instead but either
//         // we would have to recompute it in each iteration of the nested for loop
//         // or we would have to allocate to save the results
//         let triangle1 = Triangle::from([
//             transform2
//                 .rotation
//                 .inverse()
//                 .mul_vec3(a1 - transform2.translation),
//             transform2
//                 .rotation
//                 .inverse()
//                 .mul_vec3(b1 - transform2.translation),
//             transform2
//                 .rotation
//                 .inverse()
//                 .mul_vec3(c1 - transform2.translation),
//         ]);
//         let [a1, b1, c1] = [a1.truncate(), b1.truncate(), c1.truncate()];

//         let mut iter2 = vertices2.chunks_exact(3);
//         while let Some(&[a2, b2, c2]) = iter2.next() {
//             let triangle2 = TriangleXY::from([
//                 Vec3::from(a2).truncate(),
//                 Vec3::from(b2).truncate(),
//                 Vec3::from(c2).truncate(),
//             ]);
//             if triangles_intersect(&triangle1, &triangle2) {
//                 return true;
//             }
//         }
//     }

//     false
// }

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

    t >= 0.0 && u >= 0.0 && t <= rs && u <= rs
}

fn point_in_transformed_triangles(
    point: &Transform,
    triangles_transform: &Transform,
    vertices: &Vec<[f32; 3]>,
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
    t1: &Transform,
    t2: &Transform,
    vertices1: &Vec<[f32; 3]>,
    vertices2: &Vec<[f32; 3]>,
) -> bool {
    let mut iter1 = vertices1.chunks_exact(3);
    while let Some(&[a1, b1, c1]) = iter1.next() {
        // Apply t1 to triangle1
        let [a1, b1, c1] = [
            t1.transform_point(Vec3::from(a1)),
            t1.transform_point(Vec3::from(b1)),
            t1.transform_point(Vec3::from(c1)),
        ];

        // Apply t2 inverse to triangle1.
        // We could apply t2 to triangle2 instead but either
        // we would have to recompute it in each iteration of the nested for loop
        // or we would have to allocate to save the results
        let [a1, b1, c1] = [
            t2.rotation.inverse().mul_vec3(a1 - t2.translation),
            t2.rotation.inverse().mul_vec3(b1 - t2.translation),
            t2.rotation.inverse().mul_vec3(c1 - t2.translation),
        ];
        let [a1, b1, c1] = [a1.truncate(), b1.truncate(), c1.truncate()];

        let mut iter2 = vertices2.chunks_exact(3);
        while let Some(&[a2, b2, c2]) = iter2.next() {
            let [a2, b2, c2] = [
                Vec3::from(a2).truncate(),
                Vec3::from(b2).truncate(),
                Vec3::from(c2).truncate(),
            ];
            if triangles_intersect([a1, b1, c1], [a2, b2, c2]) {
                return true;
            }
        }
    }

    false
}

fn circle_intersects_transformed_triangles(
    circle: &Transform,
    radius: f32,
    triangles_transform: &Transform,
    vertices: &Vec<[f32; 3]>,
) -> bool {
    let mut iter = vertices.chunks_exact(3);
    while let Some(&[a, b, c]) = iter.next() {
        if circle_intersects_triangle(
            triangles_transform
                .rotation
                .inverse()
                .mul_vec3(circle.translation - triangles_transform.translation)
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
    t1: &Transform,
    t2: &Transform,
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
        (_, _, Topology::Point, Topology::Circle { radius })
        | (_, _, Topology::Circle { radius }, Topology::Point) => {
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
        (_, _, Topology::Circle { radius: radius1 }, Topology::Circle { radius: radius2 }) => {
            t1.translation.distance(t2.translation) < radius1 + radius2
        }
        (circle, triangles, Topology::Circle { radius }, Topology::Triangles { mesh_handle })
        | (triangles, circle, Topology::Triangles { mesh_handle }, Topology::Circle { radius }) => {
            if let Some(VertexAttributeValues::Float32x3(vertices)) = meshes
                .unwrap()
                .get(&mesh_handle.0)
                .unwrap()
                .attribute(Mesh::ATTRIBUTE_POSITION)
            {
                circle_intersects_transformed_triangles(circle, *radius, triangles, vertices)
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
