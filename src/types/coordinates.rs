use serde::{Deserialize, Serialize};

/// 3D coordinates in space
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Position {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn origin() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn distance_to(&self, other: &Position) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    pub fn distance_to_origin(&self) -> f64 {
        self.distance_to(&Position::origin())
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::origin()
    }
}

/// Galactic coordinates using polar-like system
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GalacticCoordinates {
    /// Distance from galactic center in light years
    pub radius: f64,
    /// Angle around the galactic plane (0-2π)
    pub theta: f64,
    /// Height above/below galactic plane in light years
    pub height: f64,
}

impl GalacticCoordinates {
    pub fn new(radius: f64, theta: f64, height: f64) -> Self {
        Self { radius, theta, height }
    }

    pub fn to_cartesian(&self) -> Position {
        Position {
            x: self.radius * self.theta.cos(),
            y: self.radius * self.theta.sin(),
            z: self.height,
        }
    }

    pub fn from_cartesian(pos: &Position) -> Self {
        let radius = (pos.x * pos.x + pos.y * pos.y).sqrt();
        let theta = pos.y.atan2(pos.x);
        Self {
            radius,
            theta: if theta < 0.0 { theta + std::f64::consts::TAU } else { theta },
            height: pos.z,
        }
    }
}
