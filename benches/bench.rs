use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sqr_heap::sqr_heap::SqrHeap;
use std::collections::BinaryHeap;

pub fn criterion_benchmark(c: &mut Criterion) {
  let mut sh = SqrHeap::new();
  c.bench_function("sqr_heap_push", |b| {
    b.iter(|| {
      sh.push(black_box(1));
    })
  });
  let mut bh = BinaryHeap::new();
  c.bench_function("bh_heap_push", |b| {
    b.iter(|| {
      bh.push(black_box(1));
    })
  });
}

criterion_group!(bench, criterion_benchmark);
criterion_main!(bench);
