use crate::Surround;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

impl Point3D {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

impl From<(usize, usize)> for Point3D {
    fn from(tuple: (usize, usize)) -> Self {
        Self {
            x: tuple.0 as i32,
            y: tuple.1 as i32,
            z: 0,
        }
    }
}

impl Surround for Point3D {
    fn get_surroundings(&self) -> Vec<Self> {
        let Point3D { x, y, z } = self;
        let mut points = Vec::new();

        for i in x - 1..=x + 1 {
            for j in y - 1..=y + 1 {
                for k in z - 1..=z + 1 {
                    let point = Point3D::new(i, j, k);

                    if point == *self {
                        continue;
                    }

                    points.push(point);
                }
            }
        }

        points
    }
}
