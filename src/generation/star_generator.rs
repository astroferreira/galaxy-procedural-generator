use crate::types::{LuminosityClass, SpectralClass, Star};
use rand::Rng;

use super::NameGenerator;

/// Configuration for star generation
#[derive(Debug, Clone)]
pub struct StarGeneratorConfig {
    /// Include brown dwarfs (L, T, Y class)
    pub include_brown_dwarfs: bool,
    /// Probability of brown dwarf if included
    pub brown_dwarf_probability: f64,
    /// Minimum stellar age in billions of years
    pub min_age: f64,
    /// Maximum stellar age in billions of years
    pub max_age: f64,
}

impl Default for StarGeneratorConfig {
    fn default() -> Self {
        Self {
            include_brown_dwarfs: true,
            brown_dwarf_probability: 0.05,
            min_age: 0.1,
            max_age: 13.0,
        }
    }
}

/// Generates procedural stars
pub struct StarGenerator {
    config: StarGeneratorConfig,
    name_generator: NameGenerator,
    next_id: u64,
}

impl StarGenerator {
    pub fn new(config: StarGeneratorConfig) -> Self {
        Self {
            config,
            name_generator: NameGenerator::new(),
            next_id: 0,
        }
    }

    /// Generate a star with the given parameters
    pub fn generate<R: Rng>(&mut self, rng: &mut R, system_name: &str, index: usize) -> Star {
        let spectral_class = self.generate_spectral_class(rng);
        let luminosity_class = self.generate_luminosity_class(rng, &spectral_class);
        let (mass, radius, luminosity) = self.generate_physical_properties(rng, &spectral_class, &luminosity_class);
        let temperature = self.generate_temperature(rng, &spectral_class);
        let age = rng.gen_range(self.config.min_age..self.config.max_age);
        let metallicity = self.generate_metallicity(rng, age);

        let id = self.next_id;
        self.next_id += 1;

        Star {
            id,
            name: self.name_generator.generate_star_name(rng, system_name, index),
            mass,
            radius,
            temperature,
            luminosity,
            spectral_class,
            luminosity_class,
            age,
            metallicity,
        }
    }

    fn generate_spectral_class<R: Rng>(&self, rng: &mut R) -> SpectralClass {
        // Check for brown dwarf first
        if self.config.include_brown_dwarfs && rng.gen_bool(self.config.brown_dwarf_probability) {
            let bd_type = rng.gen_range(0..3);
            return match bd_type {
                0 => SpectralClass::L,
                1 => SpectralClass::T,
                _ => SpectralClass::Y,
            };
        }

        // Use probability weights for main sequence stars
        let roll: f64 = rng.gen();
        let mut cumulative = 0.0;

        let classes = [
            SpectralClass::O,
            SpectralClass::B,
            SpectralClass::A,
            SpectralClass::F,
            SpectralClass::G,
            SpectralClass::K,
            SpectralClass::M,
        ];

        for class in &classes {
            cumulative += class.probability_weight();
            if roll < cumulative {
                return *class;
            }
        }

        SpectralClass::M // Default to most common
    }

    fn generate_luminosity_class<R: Rng>(
        &self,
        rng: &mut R,
        spectral: &SpectralClass,
    ) -> LuminosityClass {
        // Brown dwarfs are always class V equivalent
        if matches!(spectral, SpectralClass::L | SpectralClass::T | SpectralClass::Y) {
            return LuminosityClass::V;
        }

        let roll: f64 = rng.gen();

        // Most stars are main sequence
        if roll < 0.90 {
            LuminosityClass::V
        } else if roll < 0.95 {
            LuminosityClass::IV // Subgiants
        } else if roll < 0.98 {
            LuminosityClass::III // Giants
        } else if roll < 0.995 {
            // Supergiants (very rare)
            if rng.gen_bool(0.5) {
                LuminosityClass::II
            } else {
                LuminosityClass::Ib
            }
        } else {
            LuminosityClass::VII // White dwarfs
        }
    }

    fn generate_physical_properties<R: Rng>(
        &self,
        rng: &mut R,
        spectral: &SpectralClass,
        luminosity: &LuminosityClass,
    ) -> (f64, f64, f64) {
        // Base values for main sequence stars
        let (base_mass, base_radius, base_luminosity) = match spectral {
            SpectralClass::O => (30.0, 10.0, 100000.0),
            SpectralClass::B => (8.0, 4.0, 1000.0),
            SpectralClass::A => (2.0, 1.7, 20.0),
            SpectralClass::F => (1.3, 1.3, 4.0),
            SpectralClass::G => (1.0, 1.0, 1.0),
            SpectralClass::K => (0.7, 0.8, 0.3),
            SpectralClass::M => (0.3, 0.4, 0.01),
            SpectralClass::L => (0.08, 0.1, 0.0001),
            SpectralClass::T => (0.05, 0.09, 0.00001),
            SpectralClass::Y => (0.02, 0.08, 0.000001),
        };

        // Adjust for luminosity class
        let (mass_mult, radius_mult, lum_mult) = match luminosity {
            LuminosityClass::Ia0 => (50.0, 1000.0, 1000000.0),
            LuminosityClass::Ia => (25.0, 500.0, 100000.0),
            LuminosityClass::Ib => (15.0, 200.0, 50000.0),
            LuminosityClass::II => (10.0, 100.0, 10000.0),
            LuminosityClass::III => (3.0, 25.0, 500.0),
            LuminosityClass::IV => (1.5, 2.5, 5.0),
            LuminosityClass::V => (1.0, 1.0, 1.0),
            LuminosityClass::VI => (0.8, 0.8, 0.5),
            LuminosityClass::VII => (0.6, 0.01, 0.001), // White dwarfs are tiny
        };

        // Add some random variation (±20%)
        let mut variation = || 1.0 + (rng.gen::<f64>() - 0.5) * 0.4;

        (
            base_mass * mass_mult * variation(),
            base_radius * radius_mult * variation(),
            base_luminosity * lum_mult * variation(),
        )
    }

    fn generate_temperature<R: Rng>(&self, rng: &mut R, spectral: &SpectralClass) -> f64 {
        let (min, max) = spectral.temperature_range();
        rng.gen_range(min..max)
    }

    fn generate_metallicity<R: Rng>(&self, rng: &mut R, age: f64) -> f64 {
        // Older stars tend to have lower metallicity
        let base = -0.1 * age + 0.5;
        let variation = rng.gen_range(-0.3..0.3);
        (base + variation).clamp(-2.0, 0.5)
    }
}

impl Default for StarGenerator {
    fn default() -> Self {
        Self::new(StarGeneratorConfig::default())
    }
}
