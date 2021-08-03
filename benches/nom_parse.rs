use criterion::{criterion_group, criterion_main, Criterion};
use rosolio::by_nom::NoteNom;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("nom parse C♭𝄫5", |b| {
        b.iter(|| NoteNom::parse("C♭𝄫5"))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
