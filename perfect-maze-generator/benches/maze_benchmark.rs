use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use perfect_maze_generator::PerfectMaze;

fn sized_maze(c: &mut Criterion) {
    let seed = Some(42);
    let mut group = c.benchmark_group("PerfectMaze");

    for size in [10, 20, 50] {
        group.bench_with_input(BenchmarkId::from_parameter(size),
                               &size, |b, &size| {
                b.iter(|| PerfectMaze::new(size, size, seed));
            });
    }
}

criterion_group!(benches, sized_maze);
criterion_main!(benches);