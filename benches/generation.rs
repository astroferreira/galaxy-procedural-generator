use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use galaxy_generator::{GalaxyConfig, GalaxyGenerator, GalaxyType};

fn bench_galaxy_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("galaxy_generation");

    for size in [100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("spiral", size), size, |b, &size| {
            b.iter(|| {
                let config = GalaxyConfig {
                    seed: 42,
                    target_systems: size,
                    galaxy_type: GalaxyType::Spiral,
                    ..Default::default()
                };
                let mut generator = GalaxyGenerator::new(config);
                generator.generate()
            });
        });

        group.bench_with_input(BenchmarkId::new("elliptical", size), size, |b, &size| {
            b.iter(|| {
                let config = GalaxyConfig {
                    seed: 42,
                    target_systems: size,
                    galaxy_type: GalaxyType::Elliptical,
                    ..Default::default()
                };
                let mut generator = GalaxyGenerator::new(config);
                generator.generate()
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_galaxy_generation);
criterion_main!(benches);
