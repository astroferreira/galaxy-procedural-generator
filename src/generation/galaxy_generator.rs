use crate::types::{Galaxy, GalaxyStats, GalaxyType, Position};
use noise::{NoiseFn, Perlin};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use super::{NameGenerator, SystemGenerator, SystemGeneratorConfig};

/// Configuration for galaxy generation
#[derive(Debug, Clone)]
pub struct GalaxyConfig {
    /// Seed for random generation
    pub seed: u64,
    /// Radius of the galaxy in light years
    pub radius: f64,
    /// Thickness at center in light years
    pub thickness: f64,
    /// Number of spiral arms (for spiral galaxies)
    pub arm_count: u32,
    /// How tightly wound the spiral arms are
    pub arm_tightness: f64,
    /// Width of spiral arms as fraction of radius
    pub arm_width: f64,
    /// Target number of star systems to generate
    pub target_systems: usize,
    /// Type of galaxy to generate
    pub galaxy_type: GalaxyType,
    /// Density of stars in the core (relative to arms)
    pub core_density: f64,
    /// Size of the galactic core as fraction of radius
    pub core_size: f64,
    /// System generation config
    pub system_config: SystemGeneratorConfig,
}

impl Default for GalaxyConfig {
    fn default() -> Self {
        Self {
            seed: 12345,
            radius: 50000.0,
            thickness: 1000.0,
            arm_count: 2,
            arm_tightness: 0.5,
            arm_width: 0.15,
            target_systems: 1000,
            galaxy_type: GalaxyType::Spiral,
            core_density: 3.0,
            core_size: 0.15,
            system_config: SystemGeneratorConfig::default(),
        }
    }
}

/// Main galaxy generator
pub struct GalaxyGenerator {
    config: GalaxyConfig,
    rng: ChaCha8Rng,
    perlin: Perlin,
    name_generator: NameGenerator,
    system_generator: SystemGenerator,
}

impl GalaxyGenerator {
    pub fn new(config: GalaxyConfig) -> Self {
        let rng = ChaCha8Rng::seed_from_u64(config.seed);
        let perlin = Perlin::new(config.seed as u32);
        let system_config = config.system_config.clone();

        Self {
            config,
            rng,
            perlin,
            name_generator: NameGenerator::new(),
            system_generator: SystemGenerator::new(system_config),
        }
    }

    /// Generate the galaxy
    pub fn generate(&mut self) -> Galaxy {
        let name = self.name_generator.generate_galaxy_name(&mut self.rng);

        let systems = match self.config.galaxy_type {
            GalaxyType::Spiral | GalaxyType::BarredSpiral => self.generate_spiral_galaxy(),
            GalaxyType::Elliptical => self.generate_elliptical_galaxy(),
            GalaxyType::Lenticular => self.generate_lenticular_galaxy(),
            GalaxyType::Irregular => self.generate_irregular_galaxy(),
            GalaxyType::Ring => self.generate_ring_galaxy(),
        };

        let mut galaxy = Galaxy {
            id: self.config.seed,
            name,
            galaxy_type: self.config.galaxy_type,
            radius: self.config.radius,
            thickness: self.config.thickness,
            arm_count: self.config.arm_count,
            systems,
            seed: self.config.seed,
            stats: GalaxyStats::default(),
        };

        galaxy.calculate_stats();
        galaxy
    }

    fn generate_spiral_galaxy(&mut self) -> Vec<crate::types::StarSystem> {
        let mut systems = Vec::with_capacity(self.config.target_systems);
        let mut sector_id = 0u32;

        // Generate core systems
        let core_systems = (self.config.target_systems as f64 * 0.3) as usize;
        systems.extend(self.generate_core_region(core_systems, &mut sector_id));

        // Generate arm systems
        let arm_systems_each = (self.config.target_systems - core_systems) / self.config.arm_count as usize;
        for arm in 0..self.config.arm_count {
            systems.extend(self.generate_spiral_arm(arm, arm_systems_each, &mut sector_id));
        }

        // Fill in some disk systems between arms
        let remaining = self.config.target_systems.saturating_sub(systems.len());
        systems.extend(self.generate_disk_systems(remaining, &mut sector_id));

        systems
    }

    fn generate_core_region(
        &mut self,
        count: usize,
        sector_id: &mut u32,
    ) -> Vec<crate::types::StarSystem> {
        let mut systems = Vec::with_capacity(count);
        let core_radius = self.config.radius * self.config.core_size;

        for _ in 0..count {
            // Gaussian-like distribution for core
            let r = self.rng.gen::<f64>().sqrt() * core_radius;
            let theta = self.rng.gen::<f64>() * std::f64::consts::TAU;
            let z = (self.rng.gen::<f64>() - 0.5) * self.config.thickness * 0.3;

            let position = Position::new(r * theta.cos(), r * theta.sin(), z);

            let seed = self.rng.gen();
            *sector_id += 1;
            systems.push(self.system_generator.generate(&mut self.rng, position, *sector_id, seed));
        }

        systems
    }

    fn generate_spiral_arm(
        &mut self,
        arm_index: u32,
        count: usize,
        sector_id: &mut u32,
    ) -> Vec<crate::types::StarSystem> {
        let mut systems = Vec::with_capacity(count);
        let arm_offset = (arm_index as f64 / self.config.arm_count as f64) * std::f64::consts::TAU;

        let is_barred = self.config.galaxy_type == GalaxyType::BarredSpiral;
        let bar_length = if is_barred {
            self.config.radius * 0.3
        } else {
            0.0
        };

        for _ in 0..count {
            // Distance from center (start after core)
            let r_normalized = self.rng.gen::<f64>().sqrt();
            let r = self.config.radius * self.config.core_size
                + r_normalized * (self.config.radius * (1.0 - self.config.core_size));

            // Spiral angle increases with distance
            let spiral_angle = arm_offset + self.config.arm_tightness * (r / self.config.radius) * std::f64::consts::TAU;

            // Add noise to arm position
            let noise_val = self.perlin.get([r * 0.001, spiral_angle, 0.0]) as f64;
            let arm_spread = self.config.arm_width * self.config.radius;
            let perpendicular_offset = noise_val * arm_spread
                + (self.rng.gen::<f64>() - 0.5) * arm_spread * 0.5;

            // Calculate final position
            let theta = spiral_angle + perpendicular_offset / r;

            // Handle barred spiral center
            let (x, y) = if is_barred && r < bar_length {
                // Stars in bar region
                let bar_angle = arm_offset;
                let bar_offset = (self.rng.gen::<f64>() - 0.5) * self.config.thickness;
                (r * bar_angle.cos() + bar_offset * bar_angle.sin(),
                 r * bar_angle.sin() - bar_offset * bar_angle.cos())
            } else {
                (r * theta.cos(), r * theta.sin())
            };

            // Vertical position (disk is thinner at edges)
            let z_scale = 1.0 - (r / self.config.radius) * 0.5;
            let z = (self.rng.gen::<f64>() - 0.5) * self.config.thickness * z_scale;

            let position = Position::new(x, y, z);

            let seed = self.rng.gen();
            *sector_id += 1;
            systems.push(self.system_generator.generate(&mut self.rng, position, *sector_id, seed));
        }

        systems
    }

    fn generate_disk_systems(
        &mut self,
        count: usize,
        sector_id: &mut u32,
    ) -> Vec<crate::types::StarSystem> {
        let mut systems = Vec::with_capacity(count);

        for _ in 0..count {
            let r = self.rng.gen::<f64>().sqrt() * self.config.radius;
            let theta = self.rng.gen::<f64>() * std::f64::consts::TAU;

            let z_scale = 1.0 - (r / self.config.radius) * 0.7;
            let z = (self.rng.gen::<f64>() - 0.5) * self.config.thickness * z_scale;

            let position = Position::new(r * theta.cos(), r * theta.sin(), z);

            let seed = self.rng.gen();
            *sector_id += 1;
            systems.push(self.system_generator.generate(&mut self.rng, position, *sector_id, seed));
        }

        systems
    }

    fn generate_elliptical_galaxy(&mut self) -> Vec<crate::types::StarSystem> {
        let mut systems = Vec::with_capacity(self.config.target_systems);
        let mut sector_id = 0u32;

        // Elliptical galaxies have more uniform distribution
        let ellipticity = self.rng.gen_range(0.3..0.9);

        for _ in 0..self.config.target_systems {
            // 3D ellipsoid distribution
            let u = self.rng.gen::<f64>();
            let v = self.rng.gen::<f64>();
            let w = self.rng.gen::<f64>();

            // Use inverse transform for elliptical distribution
            let r = self.config.radius * u.powf(1.0 / 3.0);
            let theta = v * std::f64::consts::TAU;
            let phi = (2.0 * w - 1.0).acos();

            let x = r * phi.sin() * theta.cos();
            let y = r * phi.sin() * theta.sin() * ellipticity;
            let z = r * phi.cos() * ellipticity * 0.7;

            let position = Position::new(x, y, z);

            let seed = self.rng.gen();
            sector_id += 1;
            systems.push(self.system_generator.generate(&mut self.rng, position, sector_id, seed));
        }

        systems
    }

    fn generate_lenticular_galaxy(&mut self) -> Vec<crate::types::StarSystem> {
        let mut systems = Vec::with_capacity(self.config.target_systems);
        let mut sector_id = 0u32;

        // Lenticular: disk shape without spiral arms
        // Core bulge + disk

        let bulge_systems = (self.config.target_systems as f64 * 0.4) as usize;
        systems.extend(self.generate_core_region(bulge_systems, &mut sector_id));

        // Disk without spiral structure
        for _ in 0..(self.config.target_systems - bulge_systems) {
            let r = self.rng.gen::<f64>().sqrt() * self.config.radius;
            let theta = self.rng.gen::<f64>() * std::f64::consts::TAU;

            // Thin disk
            let z_scale = (-r / (self.config.radius * 0.3)).exp();
            let z = (self.rng.gen::<f64>() - 0.5) * self.config.thickness * 0.2 * z_scale;

            let position = Position::new(r * theta.cos(), r * theta.sin(), z);

            let seed = self.rng.gen();
            sector_id += 1;
            systems.push(self.system_generator.generate(&mut self.rng, position, sector_id, seed));
        }

        systems
    }

    fn generate_irregular_galaxy(&mut self) -> Vec<crate::types::StarSystem> {
        let mut systems = Vec::with_capacity(self.config.target_systems);
        let mut sector_id = 0u32;

        // Irregular galaxies have chaotic, asymmetric structure
        for _ in 0..self.config.target_systems {
            // Multiple overlapping noise functions for chaos
            let base_x = self.rng.gen::<f64>() * 2.0 - 1.0;
            let base_y = self.rng.gen::<f64>() * 2.0 - 1.0;
            let base_z = self.rng.gen::<f64>() * 2.0 - 1.0;

            let noise1 = self.perlin.get([base_x * 2.0, base_y * 2.0, base_z * 2.0]) as f64;
            let noise2 = self.perlin.get([base_x * 4.0, base_y * 4.0, base_z * 4.0]) as f64;

            let x = (base_x + noise1 * 0.3) * self.config.radius * 0.7;
            let y = (base_y + noise2 * 0.3) * self.config.radius * 0.7;
            let z = base_z * self.config.thickness * (1.0 + noise1.abs());

            let position = Position::new(x, y, z);

            let seed = self.rng.gen();
            sector_id += 1;
            systems.push(self.system_generator.generate(&mut self.rng, position, sector_id, seed));
        }

        systems
    }

    fn generate_ring_galaxy(&mut self) -> Vec<crate::types::StarSystem> {
        let mut systems = Vec::with_capacity(self.config.target_systems);
        let mut sector_id = 0u32;

        // Small central region
        let core_systems = (self.config.target_systems as f64 * 0.15) as usize;
        let core_radius = self.config.radius * 0.1;

        for _ in 0..core_systems {
            let r = self.rng.gen::<f64>().sqrt() * core_radius;
            let theta = self.rng.gen::<f64>() * std::f64::consts::TAU;
            let z = (self.rng.gen::<f64>() - 0.5) * self.config.thickness * 0.5;

            let position = Position::new(r * theta.cos(), r * theta.sin(), z);

            let seed = self.rng.gen();
            sector_id += 1;
            systems.push(self.system_generator.generate(&mut self.rng, position, sector_id, seed));
        }

        // Ring structure
        let ring_inner = self.config.radius * 0.6;
        let ring_outer = self.config.radius;
        let ring_width = ring_outer - ring_inner;

        for _ in 0..(self.config.target_systems - core_systems) {
            let r = ring_inner + self.rng.gen::<f64>() * ring_width;
            let theta = self.rng.gen::<f64>() * std::f64::consts::TAU;

            // Add some noise to the ring
            let noise = self.perlin.get([theta * 3.0, 0.0, 0.0]) as f64;
            let r_adjusted = r + noise * ring_width * 0.2;

            let z = (self.rng.gen::<f64>() - 0.5) * self.config.thickness * 0.3;

            let position = Position::new(r_adjusted * theta.cos(), r_adjusted * theta.sin(), z);

            let seed = self.rng.gen();
            sector_id += 1;
            systems.push(self.system_generator.generate(&mut self.rng, position, sector_id, seed));
        }

        systems
    }
}

impl Default for GalaxyGenerator {
    fn default() -> Self {
        Self::new(GalaxyConfig::default())
    }
}
