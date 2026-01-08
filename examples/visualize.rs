//! Example that generates a PNG visualization of the galaxy

use galaxy_generator::{GalaxyConfig, GalaxyGenerator, GalaxyType, SpectralClass};
use image::{Rgb, RgbImage};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let seed: u64 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(42);
    let num_systems: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(5000);
    let galaxy_type_str = args.get(3).map(|s| s.as_str()).unwrap_or("spiral");
    let output_file = args.get(4).map(|s| s.as_str()).unwrap_or("galaxy.png");

    let galaxy_type = match galaxy_type_str.to_lowercase().as_str() {
        "spiral" => GalaxyType::Spiral,
        "barred" | "barredspiral" => GalaxyType::BarredSpiral,
        "elliptical" => GalaxyType::Elliptical,
        "lenticular" => GalaxyType::Lenticular,
        "irregular" => GalaxyType::Irregular,
        "ring" => GalaxyType::Ring,
        _ => GalaxyType::Spiral,
    };

    println!("Generating {:?} galaxy with {} systems...", galaxy_type, num_systems);

    let config = GalaxyConfig {
        seed,
        target_systems: num_systems,
        galaxy_type,
        radius: 50000.0,
        ..Default::default()
    };

    let mut generator = GalaxyGenerator::new(config.clone());
    let galaxy = generator.generate();

    println!("Galaxy generated: {}", galaxy.name);
    println!("Total systems: {}", galaxy.system_count());

    // Create image
    let img_size = 2048u32;
    let mut img = RgbImage::new(img_size, img_size);

    // Fill with dark background
    for pixel in img.pixels_mut() {
        *pixel = Rgb([5, 5, 15]);
    }

    // Scale factor to fit galaxy in image
    let scale = (img_size as f64 / 2.0) / (config.radius * 1.1);
    let center = img_size as f64 / 2.0;

    // Draw stars
    for system in &galaxy.systems {
        let x = (system.position.x * scale + center) as i32;
        let y = (system.position.y * scale + center) as i32;

        // Skip if outside image bounds
        if x < 0 || x >= img_size as i32 || y < 0 || y >= img_size as i32 {
            continue;
        }

        // Get star color from primary star
        let color = if let Some(star) = system.primary_star() {
            let (r, g, b) = star.spectral_class.color();
            // Adjust brightness based on luminosity
            let brightness = (star.luminosity.log10() + 3.0).clamp(0.3, 1.5) as f32;
            Rgb([
                (r as f32 * brightness).min(255.0) as u8,
                (g as f32 * brightness).min(255.0) as u8,
                (b as f32 * brightness).min(255.0) as u8,
            ])
        } else {
            Rgb([255, 255, 255])
        };

        // Draw the star (potentially with glow for brighter stars)
        let x = x as u32;
        let y = y as u32;
        img.put_pixel(x, y, color);

        // Add slight glow for brighter stars
        if let Some(star) = system.primary_star() {
            if star.luminosity > 10.0 {
                // Draw a small cross for bright stars
                let glow_color = Rgb([color[0] / 2, color[1] / 2, color[2] / 2]);
                if x > 0 { img.put_pixel(x - 1, y, blend_colors(img.get_pixel(x - 1, y), &glow_color)); }
                if x < img_size - 1 { img.put_pixel(x + 1, y, blend_colors(img.get_pixel(x + 1, y), &glow_color)); }
                if y > 0 { img.put_pixel(x, y - 1, blend_colors(img.get_pixel(x, y - 1), &glow_color)); }
                if y < img_size - 1 { img.put_pixel(x, y + 1, blend_colors(img.get_pixel(x, y + 1), &glow_color)); }
            }
        }
    }

    // Add some label info
    println!("Saving to {}...", output_file);
    img.save(output_file).expect("Failed to save image");
    println!("Done! Galaxy visualization saved to {}", output_file);

    // Print stats
    println!("\nGalaxy Statistics:");
    println!("  Type: {:?}", galaxy.galaxy_type);
    println!("  Systems: {}", galaxy.stats.total_stars);
    println!("  Planets: {}", galaxy.stats.total_planets);
    println!("  Habitable: {}", galaxy.stats.habitable_planets);

    // Count star types
    let mut star_counts = std::collections::HashMap::new();
    for system in &galaxy.systems {
        for star in &system.stars {
            *star_counts.entry(star.spectral_class).or_insert(0) += 1;
        }
    }

    println!("\nStar Distribution:");
    for class in [
        SpectralClass::O, SpectralClass::B, SpectralClass::A, SpectralClass::F,
        SpectralClass::G, SpectralClass::K, SpectralClass::M,
    ] {
        let count = star_counts.get(&class).unwrap_or(&0);
        println!("  {:?}: {}", class, count);
    }

    println!("\nUsage: {} [seed] [num_systems] [galaxy_type] [output.png]", args[0]);
}

fn blend_colors(base: &Rgb<u8>, overlay: &Rgb<u8>) -> Rgb<u8> {
    Rgb([
        base[0].saturating_add(overlay[0]),
        base[1].saturating_add(overlay[1]),
        base[2].saturating_add(overlay[2]),
    ])
}
