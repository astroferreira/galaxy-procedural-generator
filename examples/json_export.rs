//! Example showing JSON serialization of generated galaxies

use galaxy_generator::{GalaxyConfig, GalaxyGenerator, GalaxyType};

fn main() {
    let config = GalaxyConfig {
        seed: 12345,
        target_systems: 10, // Small for demonstration
        galaxy_type: GalaxyType::Elliptical,
        ..Default::default()
    };

    let mut generator = GalaxyGenerator::new(config);
    let galaxy = generator.generate();

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&galaxy).expect("Failed to serialize galaxy");

    println!("Galaxy JSON (truncated):");
    println!("{}", &json[..json.len().min(2000)]);
    println!("...");
    println!("\nTotal JSON size: {} bytes", json.len());

    // You can also serialize just a single system
    if let Some(system) = galaxy.systems.first() {
        let system_json = serde_json::to_string_pretty(system).expect("Failed to serialize system");
        println!("\n\nFirst Star System:");
        println!("{}", system_json);
    }
}
