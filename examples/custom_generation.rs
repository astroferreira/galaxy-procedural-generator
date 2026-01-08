//! Example showing custom generation with specific configurations

use galaxy_generator::{
    GalaxyConfig, GalaxyGenerator, GalaxyType, MoonGeneratorConfig, PlanetGeneratorConfig,
    StarGeneratorConfig, SystemGeneratorConfig,
};

fn main() {
    // Configure star generation
    let star_config = StarGeneratorConfig {
        include_brown_dwarfs: false, // No brown dwarfs
        brown_dwarf_probability: 0.0,
        min_age: 1.0,  // Minimum 1 billion years old
        max_age: 10.0, // Maximum 10 billion years old
    };

    // Configure moon generation
    let moon_config = MoonGeneratorConfig {
        max_moons_rocky: 2,
        max_moons_gas: 15,
        subsurface_ocean_probability: 0.25, // Higher chance of subsurface oceans
    };

    // Configure planet generation
    let planet_config = PlanetGeneratorConfig {
        min_planets: 2,
        max_planets: 10,
        ring_probability: 0.2, // More rings
        moon_config,
    };

    // Configure system generation
    let system_config = SystemGeneratorConfig {
        binary_probability: 0.5, // More binary systems
        trinary_probability: 0.1,
        star_config,
        planet_config,
    };

    // Create galaxy config
    let galaxy_config = GalaxyConfig {
        seed: 99999,
        radius: 30000.0,
        thickness: 500.0,
        arm_count: 2,
        arm_tightness: 0.7,
        arm_width: 0.2,
        target_systems: 200,
        galaxy_type: GalaxyType::BarredSpiral,
        core_density: 4.0,
        core_size: 0.2,
        system_config,
    };

    let mut generator = GalaxyGenerator::new(galaxy_config);
    let galaxy = generator.generate();

    println!("Custom Barred Spiral Galaxy: {}", galaxy.name);
    println!("Binary/Multiple systems: {}", galaxy.stats.binary_systems);
    println!(
        "Average planets per system: {:.2}",
        galaxy.stats.average_planets_per_system
    );

    // Count moons with subsurface oceans
    let ocean_moons: usize = galaxy
        .systems
        .iter()
        .flat_map(|s| &s.planets)
        .flat_map(|p| &p.moons)
        .filter(|m| m.has_subsurface_ocean)
        .count();

    println!("Moons with subsurface oceans: {}", ocean_moons);
}
