use std::{cell::Cell, ops::{Add, AddAssign, Mul, Neg, Sub}};

pub struct DirectedLineSegment {
    pub initial_point: Vector,
    pub terminal_point: Vector
}

impl DirectedLineSegment {
    pub fn new(initial_point: Vector, terminal_point: Vector) -> DirectedLineSegment {
        DirectedLineSegment {
            initial_point,
            terminal_point
        }
    }
    pub fn relative_vector(&self) -> Vector {
        self.terminal_point - self.initial_point
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Vector {
        Vector {
            x,
            y
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        (&self).sub(&other)
    }
}

impl Sub for &Vector {
    type Output = Vector;

    fn sub(self, other: &Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Vector) {
        self.add_assign(&other)
    }
}

impl AddAssign<&Vector> for Vector {
    fn add_assign(&mut self, other: &Vector) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        &self + &other
    }
}

impl Add for &Vector {
    type Output = Vector;

    fn add(self, other: &Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        -&self
    }
}

impl Neg for &Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y
        }
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, other: Vector) -> Vector {
        &other * &self
    }
}

impl Mul<&Vector> for &f64 {
    type Output = Vector;

    fn mul(self, other: &Vector) -> Vector {
        other * self
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, other: f64) -> Vector {
        &self * &other
    }
}

impl Mul<&f64> for &Vector {
    type Output = Vector;

    fn mul(self, other: &f64) -> Vector {
        Vector {
            x: self.x * other,
            y: self.y * other
        }
    }
}
