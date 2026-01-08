//! # Galaxy Procedural Generator
//!
//! A procedural generation library for creating realistic galaxies, star systems,
//! planets, and moons for games and simulations.
//!
//! ## Features
//!
//! - Multiple galaxy types (Spiral, Barred Spiral, Elliptical, Lenticular, Irregular, Ring)
//! - Realistic star classification (O, B, A, F, G, K, M spectral classes)
//! - Diverse planet types (Terrestrial, Gas Giant, Ice Giant, Ocean worlds, etc.)
//! - Moon generation with various types (Captured, Icy, Volcanic, Habitable)
//! - Deterministic generation with seed-based reproducibility
//! - Configurable parameters for customization
//! - Serialization support with serde
//!
//! ## Quick Start
//!
//! ```rust
//! use galaxy_generator::{GalaxyGenerator, GalaxyConfig, GalaxyType};
//!
//! // Create a spiral galaxy with 1000 star systems
//! let config = GalaxyConfig {
//!     seed: 42,
//!     target_systems: 1000,
//!     galaxy_type: GalaxyType::Spiral,
//!     ..Default::default()
//! };
//!
//! let mut generator = GalaxyGenerator::new(config);
//! let galaxy = generator.generate();
//!
//! println!("Generated {} star systems", galaxy.system_count());
//! println!("Habitable worlds: {}", galaxy.stats.habitable_planets);
//! ```
//!
//! ## Architecture
//!
//! The library is organized into two main modules:
//!
//! - `types`: Core data structures (Galaxy, StarSystem, Star, Planet, Moon)
//! - `generation`: Procedural generation algorithms and configuration

pub mod generation;
pub mod types;

// Re-export commonly used types at the crate root
pub use generation::{
    GalaxyConfig, GalaxyGenerator, MoonGenerator, MoonGeneratorConfig, NameGenerator,
    PlanetGenerator, PlanetGeneratorConfig, StarGenerator, StarGeneratorConfig, SystemGenerator,
    SystemGeneratorConfig,
};

pub use types::{
    AtmosphereType, Galaxy, GalacticCoordinates, GalaxyStats, GalaxyType, LuminosityClass, Moon,
    MoonType, Planet, PlanetType, Position, SpectralClass, Star, StarSystem, SystemType,
};

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::generation::*;
    pub use crate::types::*;
}
