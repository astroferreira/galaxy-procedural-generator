use crate::types::{AtmosphereType, Planet, PlanetType, Star};
use rand::Rng;

use super::{MoonGenerator, MoonGeneratorConfig, NameGenerator};

/// Configuration for planet generation
#[derive(Debug, Clone)]
pub struct PlanetGeneratorConfig {
    /// Minimum number of planets per system
    pub min_planets: usize,
    /// Maximum number of planets per system
    pub max_planets: usize,
    /// Probability of a planet having rings
    pub ring_probability: f64,
    /// Moon generation config
    pub moon_config: MoonGeneratorConfig,
}

impl Default for PlanetGeneratorConfig {
    fn default() -> Self {
        Self {
            min_planets: 0,
            max_planets: 12,
            ring_probability: 0.1,
            moon_config: MoonGeneratorConfig::default(),
        }
    }
}

/// Generates procedural planets
pub struct PlanetGenerator {
    config: PlanetGeneratorConfig,
    name_generator: NameGenerator,
    moon_generator: MoonGenerator,
    next_id: u64,
}

impl PlanetGenerator {
    pub fn new(config: PlanetGeneratorConfig) -> Self {
        let moon_config = config.moon_config.clone();
        Self {
            config,
            name_generator: NameGenerator::new(),
            moon_generator: MoonGenerator::new(moon_config),
            next_id: 0,
        }
    }

    /// Generate planets for a star system
    pub fn generate_system_planets<R: Rng>(
        &mut self,
        rng: &mut R,
        star: &Star,
        system_name: &str,
    ) -> Vec<Planet> {
        // Determine number of planets based on star type and metallicity
        let planet_factor = self.calculate_planet_factor(star);
        let num_planets = (rng.gen_range(self.config.min_planets..=self.config.max_planets) as f64
            * planet_factor) as usize;

        let mut planets = Vec::with_capacity(num_planets);
        let mut current_distance = self.calculate_initial_distance(star);

        for i in 0..num_planets {
            let planet = self.generate_planet(rng, star, system_name, i, current_distance);
            current_distance = planet.orbital_distance * rng.gen_range(1.4..2.2); // Titius-Bode-like spacing
            planets.push(planet);
        }

        planets
    }

    fn calculate_planet_factor(&self, star: &Star) -> f64 {
        // Higher metallicity = more planets
        let metallicity_factor = 1.0 + star.metallicity * 0.5;
        // Main sequence stars have more planets
        let luminosity_factor = if star.is_main_sequence() { 1.0 } else { 0.5 };
        metallicity_factor * luminosity_factor
    }

    fn calculate_initial_distance(&self, star: &Star) -> f64 {
        // Start closer for dimmer stars
        0.1 * star.luminosity.sqrt().max(0.1)
    }

    fn generate_planet<R: Rng>(
        &mut self,
        rng: &mut R,
        star: &Star,
        system_name: &str,
        index: usize,
        distance: f64,
    ) -> Planet {
        let frost_line = star.frost_line();
        let hab_inner = star.habitable_zone_inner();
        let hab_outer = star.habitable_zone_outer();

        let in_habitable_zone = distance >= hab_inner && distance <= hab_outer;
        let beyond_frost_line = distance > frost_line;

        let planet_type = self.determine_planet_type(rng, distance, frost_line, in_habitable_zone);
        let (mass, radius) = self.generate_mass_radius(rng, &planet_type);
        let surface_gravity = mass / (radius * radius);

        let atmosphere = self.generate_atmosphere(rng, &planet_type, surface_gravity, distance, star);
        let atmospheric_pressure = self.generate_atmospheric_pressure(rng, &atmosphere);

        let eccentricity = self.generate_eccentricity(rng, &planet_type);
        let orbital_period = (distance.powi(3) / star.mass).sqrt(); // Kepler's third law
        let rotation_period = self.generate_rotation_period(rng, &planet_type, distance);
        let axial_tilt = rng.gen_range(0.0..90.0);

        let surface_temperature = self.calculate_temperature(star, distance, &atmosphere, atmospheric_pressure);
        let has_magnetic_field = self.generate_magnetic_field(rng, &planet_type, mass, rotation_period);
        let has_rings = beyond_frost_line && rng.gen_bool(self.config.ring_probability);

        let id = self.next_id;
        self.next_id += 1;

        let name = self.name_generator.generate_planet_name(rng, system_name, index);

        // Generate moons
        let moons = self.moon_generator.generate_moons(rng, &planet_type, mass, &name);

        Planet {
            id,
            name,
            planet_type,
            mass,
            radius,
            orbital_distance: distance,
            eccentricity,
            orbital_period,
            rotation_period,
            axial_tilt,
            surface_temperature,
            atmosphere,
            atmospheric_pressure,
            has_magnetic_field,
            has_rings,
            moons,
            surface_gravity,
            in_habitable_zone,
        }
    }

    fn determine_planet_type<R: Rng>(
        &self,
        rng: &mut R,
        distance: f64,
        frost_line: f64,
        in_habitable_zone: bool,
    ) -> PlanetType {
        if distance > frost_line {
            // Beyond frost line: gas giants or ice worlds
            let roll: f64 = rng.gen();
            if roll < 0.4 {
                PlanetType::GasGiant
            } else if roll < 0.7 {
                PlanetType::IceGiant
            } else {
                PlanetType::Ice
            }
        } else if distance < 0.1 {
            // Very close: hot worlds
            let roll: f64 = rng.gen();
            if roll < 0.3 {
                PlanetType::HotJupiter
            } else if roll < 0.7 {
                PlanetType::Lava
            } else {
                PlanetType::Terrestrial
            }
        } else if in_habitable_zone {
            // Habitable zone
            let roll: f64 = rng.gen();
            if roll < 0.15 {
                PlanetType::EarthLike
            } else if roll < 0.35 {
                PlanetType::Ocean
            } else if roll < 0.55 {
                PlanetType::Terrestrial
            } else if roll < 0.75 {
                PlanetType::Greenhouse
            } else {
                PlanetType::GasGiant // Hot Jupiter strays
            }
        } else {
            // Inner system, outside habitable zone
            let roll: f64 = rng.gen();
            if roll < 0.5 {
                PlanetType::Terrestrial
            } else if roll < 0.7 {
                PlanetType::Greenhouse
            } else if roll < 0.85 {
                PlanetType::Dwarf
            } else {
                PlanetType::GasGiant
            }
        }
    }

    fn generate_mass_radius<R: Rng>(&self, rng: &mut R, planet_type: &PlanetType) -> (f64, f64) {
        let (mass_min, mass_max, radius_min, radius_max) = match planet_type {
            PlanetType::Dwarf => (0.001, 0.1, 0.1, 0.4),
            PlanetType::Terrestrial => (0.1, 2.0, 0.4, 1.5),
            PlanetType::EarthLike => (0.5, 2.0, 0.8, 1.4),
            PlanetType::Lava => (0.1, 1.5, 0.4, 1.2),
            PlanetType::Greenhouse => (0.5, 3.0, 0.8, 1.5),
            PlanetType::Ocean => (0.3, 5.0, 0.7, 2.0),
            PlanetType::Ice => (0.1, 3.0, 0.5, 1.8),
            PlanetType::IceGiant => (10.0, 50.0, 3.0, 6.0),
            PlanetType::GasGiant => (50.0, 500.0, 8.0, 15.0),
            PlanetType::HotJupiter => (100.0, 1000.0, 10.0, 20.0),
        };

        let mass = rng.gen_range(mass_min..mass_max);
        let radius = rng.gen_range(radius_min..radius_max);
        (mass, radius)
    }

    fn generate_atmosphere<R: Rng>(
        &self,
        rng: &mut R,
        planet_type: &PlanetType,
        gravity: f64,
        distance: f64,
        _star: &Star,
    ) -> AtmosphereType {
        match planet_type {
            PlanetType::Dwarf => {
                if gravity > 0.1 { AtmosphereType::Thin } else { AtmosphereType::None }
            }
            PlanetType::Terrestrial => {
                let roll: f64 = rng.gen();
                if roll < 0.3 { AtmosphereType::None }
                else if roll < 0.6 { AtmosphereType::Thin }
                else if roll < 0.8 { AtmosphereType::Toxic }
                else { AtmosphereType::Reducing }
            }
            PlanetType::EarthLike => {
                let roll: f64 = rng.gen();
                if roll < 0.7 { AtmosphereType::Breathable }
                else { AtmosphereType::Thin }
            }
            PlanetType::Lava => AtmosphereType::Toxic,
            PlanetType::Greenhouse => {
                if rng.gen_bool(0.7) { AtmosphereType::Dense } else { AtmosphereType::Corrosive }
            }
            PlanetType::Ocean => {
                let roll: f64 = rng.gen();
                if roll < 0.4 { AtmosphereType::Breathable }
                else if roll < 0.7 { AtmosphereType::Dense }
                else { AtmosphereType::Thin }
            }
            PlanetType::Ice => {
                if distance > 10.0 { AtmosphereType::None }
                else if rng.gen_bool(0.5) { AtmosphereType::Thin }
                else { AtmosphereType::Reducing }
            }
            PlanetType::GasGiant | PlanetType::IceGiant | PlanetType::HotJupiter => {
                AtmosphereType::Primordial
            }
        }
    }

    fn generate_atmospheric_pressure<R: Rng>(&self, rng: &mut R, atmosphere: &AtmosphereType) -> f64 {
        match atmosphere {
            AtmosphereType::None => 0.0,
            AtmosphereType::Thin => rng.gen_range(0.001..0.5),
            AtmosphereType::Breathable => rng.gen_range(0.5..2.0),
            AtmosphereType::Dense => rng.gen_range(2.0..100.0),
            AtmosphereType::Toxic => rng.gen_range(0.1..50.0),
            AtmosphereType::Corrosive => rng.gen_range(50.0..100.0),
            AtmosphereType::Reducing => rng.gen_range(0.5..10.0),
            AtmosphereType::Primordial => rng.gen_range(100.0..10000.0),
        }
    }

    fn generate_eccentricity<R: Rng>(&self, rng: &mut R, planet_type: &PlanetType) -> f64 {
        let max_ecc = match planet_type {
            PlanetType::HotJupiter => 0.1,
            PlanetType::GasGiant | PlanetType::IceGiant => 0.15,
            _ => 0.3,
        };
        rng.gen_range(0.0..max_ecc)
    }

    fn generate_rotation_period<R: Rng>(
        &self,
        rng: &mut R,
        planet_type: &PlanetType,
        distance: f64,
    ) -> f64 {
        if distance < 0.2 {
            // Likely tidally locked
            return distance.powi(2) * 8760.0; // ~orbital period in hours
        }

        match planet_type {
            PlanetType::GasGiant | PlanetType::IceGiant | PlanetType::HotJupiter => {
                rng.gen_range(8.0..20.0) // Fast rotation
            }
            _ => rng.gen_range(10.0..1000.0), // Rocky planets have varied rotation
        }
    }

    fn calculate_temperature(
        &self,
        star: &Star,
        distance: f64,
        atmosphere: &AtmosphereType,
        pressure: f64,
    ) -> f64 {
        // Base temperature from stellar radiation
        let base_temp = 278.0 * (star.luminosity.sqrt() / distance.sqrt());

        // Greenhouse effect
        let greenhouse_factor = match atmosphere {
            AtmosphereType::None => 1.0,
            AtmosphereType::Thin => 1.0 + 0.1 * pressure,
            AtmosphereType::Breathable => 1.1,
            AtmosphereType::Dense => 1.5 + 0.01 * pressure,
            AtmosphereType::Corrosive => 2.0 + 0.02 * pressure,
            AtmosphereType::Toxic => 1.3,
            AtmosphereType::Reducing => 1.2,
            AtmosphereType::Primordial => 1.0, // Internal heating dominates
        };

        base_temp * greenhouse_factor
    }

    fn generate_magnetic_field<R: Rng>(
        &self,
        rng: &mut R,
        planet_type: &PlanetType,
        mass: f64,
        rotation: f64,
    ) -> bool {
        if planet_type.is_gas() {
            return true; // Gas giants always have magnetic fields
        }

        // Rocky planets need mass, fast rotation, and luck
        let chance = (mass * 0.3) * (24.0 / rotation.max(1.0)) * 0.5;
        rng.gen_bool(chance.clamp(0.0, 0.8))
    }
}

impl Default for PlanetGenerator {
    fn default() -> Self {
        Self::new(PlanetGeneratorConfig::default())
    }
}
