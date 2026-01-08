use serde::{Deserialize, Serialize};

/// Type of moon
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MoonType {
    /// Small irregular captured asteroid
    Captured,
    /// Regular spherical moon
    Regular,
    /// Large moon with potential for subsurface ocean
    Icy,
    /// Volcanically active moon
    Volcanic,
    /// Large moon that could potentially support life
    Habitable,
}

/// A moon orbiting a planet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Moon {
    /// Unique identifier
    pub id: u64,
    /// Name of the moon
    pub name: String,
    /// Type of moon
    pub moon_type: MoonType,
    /// Mass relative to Earth's moon
    pub mass: f64,
    /// Radius relative to Earth's moon
    pub radius: f64,
    /// Orbital distance from planet in planetary radii
    pub orbital_distance: f64,
    /// Orbital period in Earth days
    pub orbital_period: f64,
    /// Surface temperature in Kelvin
    pub surface_temperature: f64,
    /// Whether the moon is tidally locked
    pub tidally_locked: bool,
    /// Whether the moon has a thin atmosphere
    pub has_atmosphere: bool,
    /// Whether the moon might have subsurface ocean
    pub has_subsurface_ocean: bool,
}

impl Moon {
    /// Check if the moon could potentially support life
    pub fn is_potentially_habitable(&self) -> bool {
        matches!(self.moon_type, MoonType::Habitable)
            || (self.has_subsurface_ocean && self.moon_type == MoonType::Icy)
    }

    /// Get surface gravity relative to Earth's moon
    pub fn surface_gravity(&self) -> f64 {
        self.mass / (self.radius * self.radius)
    }
}
