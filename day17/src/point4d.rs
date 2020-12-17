use crate::Surround;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Point4D {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl Point4D {
    pub fn new(x: i32, y: i32, z: i32, w: i32) -> Self {
        Self { x, y, z, w }
    }
}

impl From<(usize, usize)> for Point4D {
    fn from(tuple: (usize, usize)) -> Self {
        Self {
            x: tuple.0 as i32,
            y: tuple.1 as i32,
            z: 0,
            w: 0,
        }
    }
}

impl Surround for Point4D {
    fn get_surroundings(&self) -> Vec<Self> {
        let Self { x, y, z, w } = self;
        let mut points = Vec::new();

        for i in x - 1..=x + 1 {
            for j in y - 1..=y + 1 {
                for k in z - 1..=z + 1 {
                    for l in w - 1..=w + 1 {
                        let point = Self::new(i, j, k, l);

                        if point == *self {
                            continue;
                        }

                        points.push(point);
                    }
                }
            }
        }

        points
    }
}
