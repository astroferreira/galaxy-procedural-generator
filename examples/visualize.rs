//! Example that generates a PNG visualization of the galaxy with shader-like effects
//!
//! Features:
//! - HDR rendering with bloom/glow
//! - Nebula dust clouds
//! - Star halos based on luminosity
//! - Tone mapping for final output

use galaxy_generator::{GalaxyConfig, GalaxyGenerator, GalaxyType, SpectralClass};
use image::{Rgb, RgbImage};
use noise::{NoiseFn, Perlin, Seedable};
use std::env;

/// HDR color buffer for accumulating light
struct HdrBuffer {
    width: usize,
    height: usize,
    data: Vec<[f32; 3]>,
}

impl HdrBuffer {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![[0.0, 0.0, 0.0]; width * height],
        }
    }

    fn add_pixel(&mut self, x: i32, y: i32, color: [f32; 3]) {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            let idx = y as usize * self.width + x as usize;
            self.data[idx][0] += color[0];
            self.data[idx][1] += color[1];
            self.data[idx][2] += color[2];
        }
    }

    fn get_pixel(&self, x: usize, y: usize) -> [f32; 3] {
        if x < self.width && y < self.height {
            self.data[y * self.width + x]
        } else {
            [0.0, 0.0, 0.0]
        }
    }

    fn set_pixel(&mut self, x: usize, y: usize, color: [f32; 3]) {
        if x < self.width && y < self.height {
            self.data[y * self.width + x] = color;
        }
    }

    /// Apply gaussian blur for bloom effect
    fn gaussian_blur(&self, radius: usize) -> HdrBuffer {
        let mut result = HdrBuffer::new(self.width, self.height);
        let sigma = radius as f32 / 2.0;
        let kernel = create_gaussian_kernel(radius, sigma);

        // Horizontal pass
        let mut temp = HdrBuffer::new(self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                let mut sum = [0.0f32; 3];
                let mut weight_sum = 0.0f32;

                for (i, &weight) in kernel.iter().enumerate() {
                    let sample_x = x as i32 + i as i32 - radius as i32;
                    if sample_x >= 0 && sample_x < self.width as i32 {
                        let pixel = self.get_pixel(sample_x as usize, y);
                        sum[0] += pixel[0] * weight;
                        sum[1] += pixel[1] * weight;
                        sum[2] += pixel[2] * weight;
                        weight_sum += weight;
                    }
                }

                if weight_sum > 0.0 {
                    temp.set_pixel(x, y, [
                        sum[0] / weight_sum,
                        sum[1] / weight_sum,
                        sum[2] / weight_sum,
                    ]);
                }
            }
        }

        // Vertical pass
        for y in 0..self.height {
            for x in 0..self.width {
                let mut sum = [0.0f32; 3];
                let mut weight_sum = 0.0f32;

                for (i, &weight) in kernel.iter().enumerate() {
                    let sample_y = y as i32 + i as i32 - radius as i32;
                    if sample_y >= 0 && sample_y < self.height as i32 {
                        let pixel = temp.get_pixel(x, sample_y as usize);
                        sum[0] += pixel[0] * weight;
                        sum[1] += pixel[1] * weight;
                        sum[2] += pixel[2] * weight;
                        weight_sum += weight;
                    }
                }

                if weight_sum > 0.0 {
                    result.set_pixel(x, y, [
                        sum[0] / weight_sum,
                        sum[1] / weight_sum,
                        sum[2] / weight_sum,
                    ]);
                }
            }
        }

        result
    }

    /// Add another buffer to this one
    fn add_buffer(&mut self, other: &HdrBuffer, intensity: f32) {
        for i in 0..self.data.len() {
            self.data[i][0] += other.data[i][0] * intensity;
            self.data[i][1] += other.data[i][1] * intensity;
            self.data[i][2] += other.data[i][2] * intensity;
        }
    }

    /// Tone map and convert to RGB image
    fn to_rgb_image(&self, exposure: f32, gamma: f32) -> RgbImage {
        let mut img = RgbImage::new(self.width as u32, self.height as u32);

        for y in 0..self.height {
            for x in 0..self.width {
                let hdr = self.get_pixel(x, y);

                // Exposure tone mapping
                let r = 1.0 - (-hdr[0] * exposure).exp();
                let g = 1.0 - (-hdr[1] * exposure).exp();
                let b = 1.0 - (-hdr[2] * exposure).exp();

                // Gamma correction
                let r = r.powf(1.0 / gamma);
                let g = g.powf(1.0 / gamma);
                let b = b.powf(1.0 / gamma);

                img.put_pixel(
                    x as u32,
                    y as u32,
                    Rgb([
                        (r * 255.0).clamp(0.0, 255.0) as u8,
                        (g * 255.0).clamp(0.0, 255.0) as u8,
                        (b * 255.0).clamp(0.0, 255.0) as u8,
                    ]),
                );
            }
        }

        img
    }
}

fn create_gaussian_kernel(radius: usize, sigma: f32) -> Vec<f32> {
    let size = radius * 2 + 1;
    let mut kernel = vec![0.0f32; size];
    let s = 2.0 * sigma * sigma;

    for i in 0..size {
        let x = i as f32 - radius as f32;
        kernel[i] = (-x * x / s).exp();
    }

    kernel
}

/// Draw a soft circle (for star glow)
fn draw_soft_circle(buffer: &mut HdrBuffer, cx: f32, cy: f32, radius: f32, color: [f32; 3], intensity: f32) {
    let r_int = radius.ceil() as i32;

    for dy in -r_int..=r_int {
        for dx in -r_int..=r_int {
            let dist = ((dx * dx + dy * dy) as f32).sqrt();
            if dist <= radius {
                // Soft falloff
                let falloff = 1.0 - (dist / radius).powi(2);
                let falloff = falloff * falloff; // Quadratic falloff for softer edges

                buffer.add_pixel(
                    cx as i32 + dx,
                    cy as i32 + dy,
                    [
                        color[0] * intensity * falloff,
                        color[1] * intensity * falloff,
                        color[2] * intensity * falloff,
                    ],
                );
            }
        }
    }
}

/// Draw star with diffraction spikes (for very bright stars)
fn draw_star_spikes(buffer: &mut HdrBuffer, cx: f32, cy: f32, length: f32, color: [f32; 3], intensity: f32) {
    let num_points = (length * 2.0) as i32;

    for i in 0..num_points {
        let t = i as f32 / num_points as f32;
        let falloff = 1.0 - t;
        let dist = t * length;

        // Four spikes
        for angle in [0.0, 90.0, 45.0, 135.0_f32] {
            let rad = angle.to_radians();
            let dx = dist * rad.cos();
            let dy = dist * rad.sin();

            let spike_intensity = intensity * falloff * falloff * 0.3;

            buffer.add_pixel(
                (cx + dx) as i32,
                (cy + dy) as i32,
                [
                    color[0] * spike_intensity,
                    color[1] * spike_intensity,
                    color[2] * spike_intensity,
                ],
            );
            buffer.add_pixel(
                (cx - dx) as i32,
                (cy - dy) as i32,
                [
                    color[0] * spike_intensity,
                    color[1] * spike_intensity,
                    color[2] * spike_intensity,
                ],
            );
        }
    }
}

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

    // Create HDR buffer
    let img_size = 2048usize;
    let mut hdr_buffer = HdrBuffer::new(img_size, img_size);
    let mut bloom_buffer = HdrBuffer::new(img_size, img_size);

    // Scale factor to fit galaxy in image
    let scale = (img_size as f64 / 2.0) / (config.radius * 1.1);
    let center = img_size as f64 / 2.0;

    // === PASS 1: Render nebula/dust clouds ===
    println!("Rendering nebula clouds...");
    let perlin = Perlin::new(seed as u32);

    for y in 0..img_size {
        for x in 0..img_size {
            // Convert to galaxy coordinates
            let gx = (x as f64 - center) / scale;
            let gy = (y as f64 - center) / scale;
            let dist_from_center = (gx * gx + gy * gy).sqrt();

            // Skip if too far from galaxy
            if dist_from_center > config.radius * 1.2 {
                continue;
            }

            // Multi-octave noise for nebula
            let noise_scale = 0.00008;
            let n1 = perlin.get([gx * noise_scale, gy * noise_scale, 0.0]) as f32;
            let n2 = perlin.get([gx * noise_scale * 2.0, gy * noise_scale * 2.0, 1.0]) as f32;
            let n3 = perlin.get([gx * noise_scale * 4.0, gy * noise_scale * 4.0, 2.0]) as f32;

            let noise = (n1 + n2 * 0.5 + n3 * 0.25) / 1.75;
            let noise = (noise + 1.0) / 2.0; // Normalize to 0-1

            // Density falloff from center
            let density_falloff = 1.0 - (dist_from_center / config.radius).min(1.0) as f32;
            let density_falloff = density_falloff * density_falloff;

            // Nebula color (reddish-purple for emission nebula)
            let nebula_intensity = noise * density_falloff * 0.15;

            if nebula_intensity > 0.02 {
                // Color variation based on position
                let color_noise = perlin.get([gx * noise_scale * 0.5, gy * noise_scale * 0.5, 3.0]) as f32;
                let color_t = (color_noise + 1.0) / 2.0;

                // Blend between blue and pink/red
                let r = 0.3 + color_t * 0.4;
                let g = 0.1 + (1.0 - color_t) * 0.2;
                let b = 0.4 + (1.0 - color_t) * 0.3;

                hdr_buffer.add_pixel(x as i32, y as i32, [
                    r * nebula_intensity,
                    g * nebula_intensity,
                    b * nebula_intensity,
                ]);
            }
        }
    }

    // === PASS 2: Render stars ===
    println!("Rendering {} stars...", galaxy.systems.len());

    for system in &galaxy.systems {
        let x = (system.position.x * scale + center) as f32;
        let y = (system.position.y * scale + center) as f32;

        // Skip if outside image bounds (with margin for glow)
        if x < -50.0 || x >= (img_size + 50) as f32 || y < -50.0 || y >= (img_size + 50) as f32 {
            continue;
        }

        if let Some(star) = system.primary_star() {
            let (r, g, b) = star.spectral_class.color();
            let color = [r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0];

            // Calculate star visual properties based on luminosity
            let log_lum = star.luminosity.log10() as f32;
            let base_intensity = (log_lum + 3.0).clamp(0.5, 8.0);
            let star_radius = (log_lum + 2.0).clamp(0.5, 4.0);

            // Draw star core
            draw_soft_circle(&mut hdr_buffer, x, y, star_radius, color, base_intensity);

            // Add to bloom buffer for bright stars
            if star.luminosity > 1.0 {
                let bloom_intensity = (log_lum + 1.0).clamp(0.0, 5.0);
                draw_soft_circle(&mut bloom_buffer, x, y, star_radius * 2.0, color, bloom_intensity);
            }

            // Add diffraction spikes for very bright stars
            if star.luminosity > 100.0 {
                let spike_length = (log_lum * 3.0).clamp(5.0, 30.0);
                draw_star_spikes(&mut hdr_buffer, x, y, spike_length, color, base_intensity * 0.5);
            }
        }
    }

    // === PASS 3: Apply bloom ===
    println!("Applying bloom effect...");

    // Multi-pass bloom with different radii
    let bloom1 = bloom_buffer.gaussian_blur(8);
    let bloom2 = bloom_buffer.gaussian_blur(16);
    let bloom3 = bloom_buffer.gaussian_blur(32);

    hdr_buffer.add_buffer(&bloom1, 0.6);
    hdr_buffer.add_buffer(&bloom2, 0.4);
    hdr_buffer.add_buffer(&bloom3, 0.2);

    // === PASS 4: Add background stars ===
    println!("Adding background stars...");
    let bg_perlin = Perlin::new(seed as u32 + 1000);

    for y in 0..img_size {
        for x in 0..img_size {
            let noise = bg_perlin.get([x as f64 * 0.1, y as f64 * 0.1, 0.0]) as f32;
            if noise > 0.97 {
                let intensity = (noise - 0.97) * 10.0;
                let color_var = bg_perlin.get([x as f64 * 0.5, y as f64 * 0.5, 1.0]) as f32;
                let r = 0.8 + color_var * 0.2;
                let g = 0.8 + color_var * 0.1;
                let b = 0.9;
                hdr_buffer.add_pixel(x as i32, y as i32, [r * intensity * 0.3, g * intensity * 0.3, b * intensity * 0.3]);
            }
        }
    }

    // === PASS 5: Tone mapping and output ===
    println!("Tone mapping...");
    let img = hdr_buffer.to_rgb_image(1.5, 2.2);

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
