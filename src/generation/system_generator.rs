use crate::types::{GalacticCoordinates, Position, StarSystem, SystemType};
use rand::Rng;

use super::{
    NameGenerator, PlanetGenerator, PlanetGeneratorConfig, StarGenerator, StarGeneratorConfig,
};

/// Configuration for star system generation
#[derive(Debug, Clone)]
pub struct SystemGeneratorConfig {
    /// Probability of binary system
    pub binary_probability: f64,
    /// Probability of trinary system (given not binary)
    pub trinary_probability: f64,
    /// Star generation config
    pub star_config: StarGeneratorConfig,
    /// Planet generation config
    pub planet_config: PlanetGeneratorConfig,
}

impl Default for SystemGeneratorConfig {
    fn default() -> Self {
        Self {
            binary_probability: 0.33,
            trinary_probability: 0.05,
            star_config: StarGeneratorConfig::default(),
            planet_config: PlanetGeneratorConfig::default(),
        }
    }
}

/// Generates complete star systems
pub struct SystemGenerator {
    config: SystemGeneratorConfig,
    name_generator: NameGenerator,
    star_generator: StarGenerator,
    planet_generator: PlanetGenerator,
    next_id: u64,
}

impl SystemGenerator {
    pub fn new(config: SystemGeneratorConfig) -> Self {
        let star_config = config.star_config.clone();
        let planet_config = config.planet_config.clone();
        Self {
            config,
            name_generator: NameGenerator::new(),
            star_generator: StarGenerator::new(star_config),
            planet_generator: PlanetGenerator::new(planet_config),
            next_id: 0,
        }
    }

    /// Generate a star system at the given position
    pub fn generate<R: Rng>(
        &mut self,
        rng: &mut R,
        position: Position,
        sector_id: u32,
        seed: u64,
    ) -> StarSystem {
        let name = self.name_generator.generate_system_name(rng);
        let system_type = self.determine_system_type(rng);
        let num_stars = self.num_stars_for_type(&system_type);

        // Generate stars
        let mut stars = Vec::with_capacity(num_stars);
        for i in 0..num_stars {
            stars.push(self.star_generator.generate(rng, &name, i));
        }

        // Generate planets around the primary star
        let planets = if let Some(primary) = stars.first() {
            self.planet_generator
                .generate_system_planets(rng, primary, &name)
        } else {
            Vec::new()
        };

        let galactic_coords = GalacticCoordinates::from_cartesian(&position);

        let id = self.next_id;
        self.next_id += 1;

        StarSystem {
            id,
            name,
            position,
            galactic_coords,
            system_type,
            stars,
            planets,
            explored: false,
            seed,
            sector_id,
        }
    }

    fn determine_system_type<R: Rng>(&self, rng: &mut R) -> SystemType {
        let roll: f64 = rng.gen();

        if roll < self.config.binary_probability {
            SystemType::Binary
        } else if roll < self.config.binary_probability + self.config.trinary_probability {
            SystemType::Trinary
        } else if roll > 0.99 {
            SystemType::Multiple
        } else {
            SystemType::Single
        }
    }

    fn num_stars_for_type(&self, system_type: &SystemType) -> usize {
        match system_type {
            SystemType::Single => 1,
            SystemType::Binary => 2,
            SystemType::Trinary => 3,
            SystemType::Multiple => 4,
        }
    }
}

impl Default for SystemGenerator {
    fn default() -> Self {
        Self::new(SystemGeneratorConfig::default())
    }
}
