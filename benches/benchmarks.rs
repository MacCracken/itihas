use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn bench_all_eras(c: &mut Criterion) {
    c.bench_function("all_eras", |b| b.iter(itihas::era::all_eras));
}

fn bench_eras_containing(c: &mut Criterion) {
    c.bench_function("eras_containing_500bce", |b| {
        b.iter(|| itihas::era::eras_containing(black_box(-500)))
    });
}

fn bench_all_civilizations(c: &mut Criterion) {
    c.bench_function("all_civilizations", |b| {
        b.iter(itihas::civilization::all_civilizations)
    });
}

fn bench_active_at(c: &mut Criterion) {
    c.bench_function("civilizations_active_at_500bce", |b| {
        b.iter(|| itihas::civilization::active_at(black_box(-500)))
    });
}

fn bench_by_region(c: &mut Criterion) {
    c.bench_function("civilizations_by_region", |b| {
        b.iter(|| itihas::civilization::by_region(black_box("Mediterranean")))
    });
}

fn bench_all_events(c: &mut Criterion) {
    c.bench_function("all_events", |b| b.iter(itihas::event::all_events));
}

criterion_group!(
    benches,
    bench_all_eras,
    bench_eras_containing,
    bench_all_civilizations,
    bench_active_at,
    bench_by_region,
    bench_all_events,
);
criterion_main!(benches);
