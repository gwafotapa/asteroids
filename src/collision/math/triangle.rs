use bevy::prelude::*;
use rand::Rng;

#[derive(Clone, Copy)]
pub struct Triangle(pub Vec3, pub Vec3, pub Vec3);

impl Triangle {
    pub fn to_array(&self) -> [Vec3; 3] {
        [self.0, self.1, self.2]
    }
    //     // fn new(a: Vec3, b: Vec3, c: Vec3) -> Triangle {
    //     //     Triangle(a, b, c)
    //     // }

    pub fn xy(&self) -> TriangleXY {
        TriangleXY(self.0.truncate(), self.1.truncate(), self.2.truncate())
    }

    // Area of CCW triangle
    pub fn area(&self) -> f32 {
        self.xy().area()
    }

    pub fn random_point(&self) -> Vec3 {
        let [a, b, c] = self.to_array();
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0.0..=1.0);
        let y = rng.gen_range(0.0..=1.0 - x);

        a + x * (b - a) + y * (c - a)
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
pub struct TriangleXY(pub Vec2, pub Vec2, pub Vec2);

impl TriangleXY {
    pub fn to_array(&self) -> [Vec2; 3] {
        [self.0, self.1, self.2]
    }

    // Area of CCW triangle
    pub fn area(&self) -> f32 {
        let [a, b, c] = self.to_array();
        (b - a).perp_dot(c - a) / 2.0 // .abs() unnecessary since triangle is CCW
    }

    pub fn random_point(&self) -> Vec2 {
        let [a, b, c] = self.to_array();
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0.0..=1.0);
        let y = rng.gen_range(0.0..=1.0 - x);

        a + x * (b - a) + y * (c - a)
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
