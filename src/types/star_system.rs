use super::{GalacticCoordinates, Planet, Position, Star};
use serde::{Deserialize, Serialize};

/// Type of star system based on number of stars
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SystemType {
    /// Single star system
    Single,
    /// Binary star system
    Binary,
    /// Triple star system
    Trinary,
    /// Four or more stars
    Multiple,
}

/// A star system containing one or more stars and their orbiting bodies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarSystem {
    /// Unique identifier
    pub id: u64,
    /// Name of the system
    pub name: String,
    /// Position in 3D space (in light years from galactic center)
    pub position: Position,
    /// Galactic coordinates
    pub galactic_coords: GalacticCoordinates,
    /// Type of system
    pub system_type: SystemType,
    /// Stars in the system
    pub stars: Vec<Star>,
    /// Planets in the system
    pub planets: Vec<Planet>,
    /// Whether the system has been explored/visited
    pub explored: bool,
    /// Seed used to generate this system
    pub seed: u64,
    /// Region/sector identifier
    pub sector_id: u32,
}

impl StarSystem {
    /// Get the primary (most massive) star
    pub fn primary_star(&self) -> Option<&Star> {
        self.stars.iter().max_by(|a, b| {
            a.mass
                .partial_cmp(&b.mass)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    /// Get total number of celestial bodies
    pub fn body_count(&self) -> usize {
        let moon_count: usize = self.planets.iter().map(|p| p.moons.len()).sum();
        self.stars.len() + self.planets.len() + moon_count
    }

    /// Check if the system has any potentially habitable worlds
    pub fn has_habitable_worlds(&self) -> bool {
        self.planets.iter().any(|p| p.is_potentially_habitable())
            || self
                .planets
                .iter()
                .flat_map(|p| &p.moons)
                .any(|m| m.is_potentially_habitable())
    }

    /// Get all planets in the habitable zone
    pub fn habitable_zone_planets(&self) -> Vec<&Planet> {
        self.planets.iter().filter(|p| p.in_habitable_zone).collect()
    }

    /// Get distance from galactic center in light years
    pub fn distance_from_center(&self) -> f64 {
        self.position.distance_to_origin()
    }

    /// Check if this is a binary or multiple star system
    pub fn is_multiple(&self) -> bool {
        self.stars.len() > 1
    }
}
