use super::StarSystem;
use serde::{Deserialize, Serialize};

/// Type of galaxy morphology
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GalaxyType {
    /// Spiral galaxy like the Milky Way
    Spiral,
    /// Barred spiral galaxy
    BarredSpiral,
    /// Elliptical galaxy
    Elliptical,
    /// Lenticular galaxy (disk without spiral arms)
    Lenticular,
    /// Irregular galaxy
    Irregular,
    /// Ring galaxy
    Ring,
}

impl GalaxyType {
    /// Get typical arm count for spiral types
    pub fn typical_arm_count(&self) -> Option<u32> {
        match self {
            GalaxyType::Spiral => Some(2),
            GalaxyType::BarredSpiral => Some(2),
            _ => None,
        }
    }

    /// Whether this galaxy type has a central bar
    pub fn has_bar(&self) -> bool {
        matches!(self, GalaxyType::BarredSpiral)
    }

    /// Whether this galaxy type has spiral arms
    pub fn has_spiral_arms(&self) -> bool {
        matches!(self, GalaxyType::Spiral | GalaxyType::BarredSpiral)
    }
}

/// Statistics about a generated galaxy
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GalaxyStats {
    pub total_stars: u64,
    pub total_planets: u64,
    pub total_moons: u64,
    pub habitable_planets: u64,
    pub binary_systems: u64,
    pub average_planets_per_system: f64,
}

/// A generated galaxy containing star systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Galaxy {
    /// Unique identifier
    pub id: u64,
    /// Name of the galaxy
    pub name: String,
    /// Type of galaxy
    pub galaxy_type: GalaxyType,
    /// Radius in light years
    pub radius: f64,
    /// Thickness at center in light years
    pub thickness: f64,
    /// Number of spiral arms (if applicable)
    pub arm_count: u32,
    /// Star systems in the galaxy
    pub systems: Vec<StarSystem>,
    /// Seed used for generation
    pub seed: u64,
    /// Generation statistics
    pub stats: GalaxyStats,
}

impl Galaxy {
    /// Get the total number of star systems
    pub fn system_count(&self) -> usize {
        self.systems.len()
    }

    /// Find systems within a given radius of a position
    pub fn systems_in_radius(&self, center: &super::Position, radius: f64) -> Vec<&StarSystem> {
        self.systems
            .iter()
            .filter(|s| s.position.distance_to(center) <= radius)
            .collect()
    }

    /// Find the nearest system to a position
    pub fn nearest_system(&self, pos: &super::Position) -> Option<&StarSystem> {
        self.systems
            .iter()
            .min_by(|a, b| {
                let dist_a = a.position.distance_to(pos);
                let dist_b = b.position.distance_to(pos);
                dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
            })
    }

    /// Get systems with habitable worlds
    pub fn habitable_systems(&self) -> Vec<&StarSystem> {
        self.systems
            .iter()
            .filter(|s| s.has_habitable_worlds())
            .collect()
    }

    /// Calculate galaxy statistics
    pub fn calculate_stats(&mut self) {
        let mut stats = GalaxyStats::default();

        for system in &self.systems {
            stats.total_stars += system.stars.len() as u64;
            stats.total_planets += system.planets.len() as u64;

            for planet in &system.planets {
                stats.total_moons += planet.moons.len() as u64;
                if planet.is_potentially_habitable() {
                    stats.habitable_planets += 1;
                }
            }

            if system.is_multiple() {
                stats.binary_systems += 1;
            }
        }

        if !self.systems.is_empty() {
            stats.average_planets_per_system =
                stats.total_planets as f64 / self.systems.len() as f64;
        }

        self.stats = stats;
    }
}
