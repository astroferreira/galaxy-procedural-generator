use galaxy_generator::{GalaxyConfig, GalaxyGenerator, GalaxyType};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Parse command line arguments
    let seed: u64 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(12345);
    let num_systems: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(100);
    let galaxy_type = args
        .get(3)
        .map(|s| match s.to_lowercase().as_str() {
            "spiral" => GalaxyType::Spiral,
            "barred" | "barredspiral" => GalaxyType::BarredSpiral,
            "elliptical" => GalaxyType::Elliptical,
            "lenticular" => GalaxyType::Lenticular,
            "irregular" => GalaxyType::Irregular,
            "ring" => GalaxyType::Ring,
            _ => GalaxyType::Spiral,
        })
        .unwrap_or(GalaxyType::Spiral);

    println!("Galaxy Procedural Generator");
    println!("===========================");
    println!();
    println!("Configuration:");
    println!("  Seed: {}", seed);
    println!("  Target Systems: {}", num_systems);
    println!("  Galaxy Type: {:?}", galaxy_type);
    println!();

    // Generate the galaxy
    let config = GalaxyConfig {
        seed,
        target_systems: num_systems,
        galaxy_type,
        ..Default::default()
    };

    let mut generator = GalaxyGenerator::new(config);

    println!("Generating galaxy...");
    let start = std::time::Instant::now();
    let galaxy = generator.generate();
    let duration = start.elapsed();

    println!("Generation completed in {:?}", duration);
    println!();

    // Print statistics
    println!("Galaxy: {}", galaxy.name);
    println!("Type: {:?}", galaxy.galaxy_type);
    println!("Radius: {:.0} light years", galaxy.radius);
    println!();

    println!("Statistics:");
    println!("  Total Star Systems: {}", galaxy.system_count());
    println!("  Total Stars: {}", galaxy.stats.total_stars);
    println!("  Total Planets: {}", galaxy.stats.total_planets);
    println!("  Total Moons: {}", galaxy.stats.total_moons);
    println!("  Habitable Planets: {}", galaxy.stats.habitable_planets);
    println!("  Binary/Multiple Systems: {}", galaxy.stats.binary_systems);
    println!(
        "  Avg Planets per System: {:.2}",
        galaxy.stats.average_planets_per_system
    );
    println!();

    // Show some sample systems
    println!("Sample Star Systems:");
    println!("-------------------");

    for (i, system) in galaxy.systems.iter().take(5).enumerate() {
        println!("\n{}. {} ({:?})", i + 1, system.name, system.system_type);
        println!(
            "   Position: ({:.1}, {:.1}, {:.1}) ly from center",
            system.position.x, system.position.y, system.position.z
        );
        println!(
            "   Distance from center: {:.1} ly",
            system.distance_from_center()
        );

        if let Some(star) = system.primary_star() {
            println!(
                "   Primary Star: {:?}{:?}, {:.2} solar masses, {:.0}K",
                star.spectral_class, star.luminosity_class, star.mass, star.temperature
            );
        }

        println!("   Planets: {}", system.planets.len());
        for planet in &system.planets {
            let hab_marker = if planet.is_potentially_habitable() {
                " [HABITABLE]"
            } else {
                ""
            };
            println!(
                "     - {} ({:?}): {:.2} AU, {:.0}K{}",
                planet.name,
                planet.planet_type,
                planet.orbital_distance,
                planet.surface_temperature,
                hab_marker
            );
            if !planet.moons.is_empty() {
                println!("       Moons: {}", planet.moons.len());
            }
        }
    }

    // Find habitable worlds
    let habitable = galaxy.habitable_systems();
    if !habitable.is_empty() {
        println!("\n\nSystems with Habitable Worlds:");
        println!("------------------------------");
        for system in habitable.iter().take(10) {
            println!("  {}", system.name);
            for planet in &system.planets {
                if planet.is_potentially_habitable() {
                    println!(
                        "    - {} ({:?}): {:.0}K, {:?} atmosphere",
                        planet.name, planet.planet_type, planet.surface_temperature, planet.atmosphere
                    );
                }
            }
        }
    }

    println!("\n\nUsage: {} [seed] [num_systems] [galaxy_type]", args[0]);
    println!("  galaxy_type: spiral, barred, elliptical, lenticular, irregular, ring");
}
