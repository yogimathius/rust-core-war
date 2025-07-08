use criterion::{criterion_group, criterion_main, Criterion};
use corewar::{GameConfig, GameEngine};

fn bench_engine_tick(c: &mut Criterion) {
    let mut group = c.benchmark_group("engine_tick");

    // Benchmark setup: Create a GameEngine with a simple champion
    let config = GameConfig::default();
    let mut engine = GameEngine::new(config);

    // Load a dummy champion (replace with a real champion for more realistic benchmarks)
    // For simplicity, we'll just set running to true and tick.
    engine.set_running(true);

    group.bench_function("tick_1000_cycles", |b| {
        b.iter(|| {
            // Run 1000 cycles
            for _ in 0..1000 {
                engine.tick().unwrap();
            }
        })
    });

    group.finish();
}

criterion_group!(benches, bench_engine_tick);
criterion_main!(benches);
