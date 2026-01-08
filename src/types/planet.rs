use super::Moon;
use serde::{Deserialize, Serialize};

/// Type of planet based on composition
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PlanetType {
    /// Small rocky planets (Mercury, Mars)
    Terrestrial,
    /// Earth-like planets with potential for life
    EarthLike,
    /// Hot rocky planets close to their star
    Lava,
    /// Rocky planets with thick atmospheres (Venus-like)
    Greenhouse,
    /// Planets covered in water/ice
    Ocean,
    /// Frozen rocky worlds
    Ice,
    /// Large gas giants (Jupiter-like)
    GasGiant,
    /// Smaller gas giants (Neptune-like)
    IceGiant,
    /// Hot gas giants very close to their star
    HotJupiter,
    /// Very small rocky bodies
    Dwarf,
}

impl PlanetType {
    pub fn is_rocky(&self) -> bool {
        matches!(
            self,
            PlanetType::Terrestrial
                | PlanetType::EarthLike
                | PlanetType::Lava
                | PlanetType::Greenhouse
                | PlanetType::Ocean
                | PlanetType::Ice
                | PlanetType::Dwarf
        )
    }

    pub fn is_gas(&self) -> bool {
        matches!(
            self,
            PlanetType::GasGiant | PlanetType::IceGiant | PlanetType::HotJupiter
        )
    }

    pub fn can_support_life(&self) -> bool {
        matches!(self, PlanetType::EarthLike | PlanetType::Ocean)
    }
}

/// Atmospheric composition type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AtmosphereType {
    None,
    Thin,
    Breathable,
    Dense,
    Toxic,
    Corrosive,
    Reducing,
    /// Hydrogen/Helium dominated (gas giants)
    Primordial,
}

/// A planet orbiting a star
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Planet {
    /// Unique identifier
    pub id: u64,
    /// Name of the planet
    pub name: String,
    /// Type of planet
    pub planet_type: PlanetType,
    /// Mass relative to Earth mass
    pub mass: f64,
    /// Radius relative to Earth radius
    pub radius: f64,
    /// Semi-major axis of orbit in AU
    pub orbital_distance: f64,
    /// Orbital eccentricity (0 = circular, 1 = parabolic)
    pub eccentricity: f64,
    /// Orbital period in Earth years
    pub orbital_period: f64,
    /// Rotation period in Earth hours
    pub rotation_period: f64,
    /// Axial tilt in degrees
    pub axial_tilt: f64,
    /// Surface temperature in Kelvin
    pub surface_temperature: f64,
    /// Atmosphere type
    pub atmosphere: AtmosphereType,
    /// Atmospheric pressure relative to Earth
    pub atmospheric_pressure: f64,
    /// Whether the planet has a magnetic field
    pub has_magnetic_field: bool,
    /// Whether the planet has rings
    pub has_rings: bool,
    /// Moons orbiting this planet
    pub moons: Vec<Moon>,
    /// Surface gravity relative to Earth
    pub surface_gravity: f64,
    /// Whether this planet is in the habitable zone
    pub in_habitable_zone: bool,
}

impl Planet {
    /// Calculate escape velocity in km/s
    pub fn escape_velocity(&self) -> f64 {
        // Earth's escape velocity is 11.186 km/s
        11.186 * (self.mass / self.radius).sqrt()
    }

    /// Get density relative to Earth
    pub fn density(&self) -> f64 {
        self.mass / (self.radius * self.radius * self.radius)
    }

    /// Check if potentially habitable
    pub fn is_potentially_habitable(&self) -> bool {
        self.in_habitable_zone
            && self.planet_type.can_support_life()
            && matches!(self.atmosphere, AtmosphereType::Breathable | AtmosphereType::Thin)
            && self.surface_temperature > 200.0
            && self.surface_temperature < 350.0
    }

    /// Get the number of moons
    pub fn moon_count(&self) -> usize {
        self.moons.len()
    }
}
