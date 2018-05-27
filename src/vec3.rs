use std::ops::{Add, Sub, Div, Mul, Neg};

#[derive(Debug, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3{x, y, z}
    }

    pub fn make_unit_vector(&mut self) {
        let k: f64 = 1.0 / (self.x * self.x + self.y * self.y + self.z * self.z);
        self.x *= k;
        self.y *= k;
        self.z *= k;
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn unit_vector(&self) -> Vec3 {
        self / self.dot(&self).sqrt()
    }

    pub fn length(&self) -> f64 {
        self.dot(self).sqrt()
    }
}

impl Clone for Vec3 {
    fn clone(&self) -> Vec3 {
        *self
    }
}

/*
 *  Add implementations for Vec3
 */
impl<'a, 'b> Add<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn add(self, other: &'b Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl<'a> Add<Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl<'a> Add<&'a Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: &'a Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

/*
 *  Subtract implementations for Vec3
 */
impl<'a, 'b> Sub<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn sub(self, other: &'b Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl<'a> Sub<&'a Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: &'a Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl<'a> Sub<Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

/*
 *  Negative implementation for Vec3
 */
impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

/*
 *  Multiplication implementations for Vec3
 */
impl<'a, 'b> Mul<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn mul(self, other: &'b Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

impl<'a> Mul<&'a Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: &'a Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

impl<'a> Mul<Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, val: f64) -> Vec3 {
        Vec3 {
            x: self.x * val,
            y: self.y * val,
            z: self.z * val
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z
        }
    }
}

impl<'a> Mul<f64> for &'a Vec3 {
    type Output = Vec3;

    fn mul(self, val: f64) -> Vec3 {
        Vec3 {
            x: self.x * val,
            y: self.y * val,
            z: self.z * val
        }
    }
}

impl<'a> Mul<&'a Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: &'a Vec3) -> Vec3 {
        Vec3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z
        }
    }
}

/*
 *  Division implementation for Vec3
 */
impl<'a, 'b> Div<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn div(self, other: &'b Vec3) -> Vec3 {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z
        }
    }
}

impl<'a> Div<&'a Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, other: &'a Vec3) -> Vec3 {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z
        }
    }
}

impl<'a> Div<Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z
        }
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z
        }
    }
}

impl<'a> Div<f64> for &'a Vec3 {
    type Output = Vec3;

    fn div(self, val: f64) -> Vec3 {
        Vec3 {
            x: self.x / val,
            y: self.y / val,
            z: self.z / val
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, val: f64) -> Vec3 {
        Vec3 {
            x: self.x / val,
            y: self.y / val,
            z: self.z / val
        }
    }
}