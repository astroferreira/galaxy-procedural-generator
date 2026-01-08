use crate::types::{Moon, MoonType, PlanetType};
use rand::Rng;

use super::NameGenerator;

/// Configuration for moon generation
#[derive(Debug, Clone)]
pub struct MoonGeneratorConfig {
    /// Maximum moons for rocky planets
    pub max_moons_rocky: usize,
    /// Maximum moons for gas giants
    pub max_moons_gas: usize,
    /// Probability of a moon having a subsurface ocean
    pub subsurface_ocean_probability: f64,
}

impl Default for MoonGeneratorConfig {
    fn default() -> Self {
        Self {
            max_moons_rocky: 3,
            max_moons_gas: 20,
            subsurface_ocean_probability: 0.15,
        }
    }
}

/// Generates procedural moons
pub struct MoonGenerator {
    config: MoonGeneratorConfig,
    name_generator: NameGenerator,
    next_id: u64,
}

impl MoonGenerator {
    pub fn new(config: MoonGeneratorConfig) -> Self {
        Self {
            config,
            name_generator: NameGenerator::new(),
            next_id: 0,
        }
    }

    /// Generate moons for a planet
    pub fn generate_moons<R: Rng>(
        &mut self,
        rng: &mut R,
        planet_type: &PlanetType,
        planet_mass: f64,
        planet_name: &str,
    ) -> Vec<Moon> {
        let max_moons = if planet_type.is_gas() {
            self.config.max_moons_gas
        } else {
            self.config.max_moons_rocky
        };

        // Number of moons scales with planet mass
        let moon_factor = if planet_type.is_gas() {
            (planet_mass / 100.0).sqrt()
        } else {
            (planet_mass).sqrt() * 0.5
        };

        let num_moons = (rng.gen_range(0..=max_moons) as f64 * moon_factor).min(max_moons as f64) as usize;

        let mut moons = Vec::with_capacity(num_moons);

        for i in 0..num_moons {
            moons.push(self.generate_moon(rng, planet_type, planet_mass, planet_name, i));
        }

        moons
    }

    fn generate_moon<R: Rng>(
        &mut self,
        rng: &mut R,
        planet_type: &PlanetType,
        planet_mass: f64,
        planet_name: &str,
        index: usize,
    ) -> Moon {
        let moon_type = self.determine_moon_type(rng, planet_type, index);
        let (mass, radius) = self.generate_mass_radius(rng, &moon_type, planet_mass);

        let orbital_distance = self.generate_orbital_distance(rng, index);
        let orbital_period = self.calculate_orbital_period(orbital_distance, planet_mass);

        let tidally_locked = orbital_distance < 20.0 || rng.gen_bool(0.7);
        let surface_temperature = self.calculate_temperature(rng, &moon_type);

        let has_atmosphere = self.determine_atmosphere(rng, &moon_type, mass);
        let has_subsurface_ocean = matches!(moon_type, MoonType::Icy)
            && rng.gen_bool(self.config.subsurface_ocean_probability);

        let id = self.next_id;
        self.next_id += 1;

        Moon {
            id,
            name: self.name_generator.generate_moon_name(rng, planet_name, index),
            moon_type,
            mass,
            radius,
            orbital_distance,
            orbital_period,
            surface_temperature,
            tidally_locked,
            has_atmosphere,
            has_subsurface_ocean,
        }
    }

    fn determine_moon_type<R: Rng>(
        &self,
        rng: &mut R,
        planet_type: &PlanetType,
        index: usize,
    ) -> MoonType {
        // First few moons of gas giants tend to be regular/icy
        if planet_type.is_gas() && index < 4 {
            let roll: f64 = rng.gen();
            if roll < 0.4 {
                return MoonType::Icy;
            } else if roll < 0.7 {
                return MoonType::Regular;
            } else if roll < 0.85 {
                return MoonType::Volcanic;
            } else {
                return MoonType::Habitable;
            }
        }

        // Outer moons are often captured
        if index > 6 {
            return MoonType::Captured;
        }

        let roll: f64 = rng.gen();
        if roll < 0.3 {
            MoonType::Captured
        } else if roll < 0.6 {
            MoonType::Regular
        } else if roll < 0.85 {
            MoonType::Icy
        } else if roll < 0.95 {
            MoonType::Volcanic
        } else {
            MoonType::Habitable
        }
    }

    fn generate_mass_radius<R: Rng>(
        &self,
        rng: &mut R,
        moon_type: &MoonType,
        planet_mass: f64,
    ) -> (f64, f64) {
        // Mass relative to Earth's moon, radius relative to Earth's moon
        let (mass_min, mass_max, radius_min, radius_max) = match moon_type {
            MoonType::Captured => (0.0001, 0.01, 0.01, 0.1),
            MoonType::Regular => (0.01, 1.0, 0.1, 1.0),
            MoonType::Icy => (0.05, 2.0, 0.2, 1.5),
            MoonType::Volcanic => (0.1, 1.5, 0.2, 1.2),
            MoonType::Habitable => (0.5, 3.0, 0.5, 1.8),
        };

        // Scale by planet mass (larger planets can have larger moons)
        let mass_scale = (planet_mass / 100.0).sqrt().clamp(0.1, 2.0);

        let mass = rng.gen_range(mass_min..mass_max) * mass_scale;
        let radius = rng.gen_range(radius_min..radius_max) * mass_scale.sqrt();
        (mass, radius)
    }

    fn generate_orbital_distance<R: Rng>(&self, rng: &mut R, index: usize) -> f64 {
        // Distance in planetary radii, increasing with index
        let base = 3.0 + (index as f64 * 5.0);
        base * rng.gen_range(0.8..1.4)
    }

    fn calculate_orbital_period(&self, distance: f64, planet_mass: f64) -> f64 {
        // Simplified orbital period in Earth days
        // Based on Kepler's third law
        let period_factor = (distance.powi(3) / planet_mass).sqrt();
        period_factor * 0.1 // Scale to reasonable day values
    }

    fn calculate_temperature<R: Rng>(&self, rng: &mut R, moon_type: &MoonType) -> f64 {
        match moon_type {
            MoonType::Volcanic => rng.gen_range(200.0..400.0),
            MoonType::Icy => rng.gen_range(50.0..150.0),
            MoonType::Habitable => rng.gen_range(200.0..320.0),
            _ => rng.gen_range(80.0..200.0),
        }
    }

    fn determine_atmosphere<R: Rng>(&self, rng: &mut R, moon_type: &MoonType, mass: f64) -> bool {
        let base_chance = match moon_type {
            MoonType::Habitable => 0.9,
            MoonType::Volcanic => 0.5,
            MoonType::Icy => 0.2,
            MoonType::Regular => 0.1,
            MoonType::Captured => 0.01,
        };

        let mass_factor = mass.sqrt();
        rng.gen_bool((base_chance * mass_factor).clamp(0.0, 0.95))
    }
}

impl Default for MoonGenerator {
    fn default() -> Self {
        Self::new(MoonGeneratorConfig::default())
    }
}
