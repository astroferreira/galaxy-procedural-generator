//! Basic usage example for the galaxy generator

use galaxy_generator::{GalaxyConfig, GalaxyGenerator, GalaxyType};

fn main() {
    // Create a configuration for a spiral galaxy
    let config = GalaxyConfig {
        seed: 42,
        radius: 50000.0,
        thickness: 1000.0,
        arm_count: 4,
        target_systems: 500,
        galaxy_type: GalaxyType::Spiral,
        ..Default::default()
    };

    // Create the generator and generate the galaxy
    let mut generator = GalaxyGenerator::new(config);
    let galaxy = generator.generate();

    println!("Generated galaxy: {}", galaxy.name);
    println!("Type: {:?}", galaxy.galaxy_type);
    println!("Star systems: {}", galaxy.system_count());
    println!("Total stars: {}", galaxy.stats.total_stars);
    println!("Total planets: {}", galaxy.stats.total_planets);
    println!("Habitable planets: {}", galaxy.stats.habitable_planets);

    // Find the nearest system to a given position
    let search_pos = galaxy_generator::Position::new(1000.0, 1000.0, 0.0);
    if let Some(nearest) = galaxy.nearest_system(&search_pos) {
        println!("\nNearest system to (1000, 1000, 0): {}", nearest.name);
    }

    // Get all systems with habitable worlds
    let habitable = galaxy.habitable_systems();
    println!("\nSystems with habitable worlds: {}", habitable.len());
}
