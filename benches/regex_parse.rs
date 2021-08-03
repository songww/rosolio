use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rosolio::by_regex::NoteRegex;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("regex parse C♭𝄫5", |b| {
        b.iter(|| NoteRegex::parse("C♭𝄫5"))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
