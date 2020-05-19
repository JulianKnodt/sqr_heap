use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use sqr_heap::sqr_heap::SqrHeap;
use std::collections::BinaryHeap;

pub fn criterion_benchmark(c: &mut Criterion) {
  let range = (0..12).map(|p| 1 << p);
  let mut group = c.benchmark_group("pop->push pair");
  for l in range {
    let mut sh = SqrHeap::new();
    let mut bh = BinaryHeap::new();
    for i in 0..l {
      sh.push(i);
      bh.push(i);
    }
    group.bench_with_input(BenchmarkId::new("Sqr Heap", l), &l, |b, _| {
      b.iter(|| {
        let out = sh.pop().unwrap();
        sh.push(out);
      })
    });
    group.bench_with_input(BenchmarkId::new("Bin Heap", l), &l, |b, _| {
      b.iter(|| {
        let out = bh.pop().unwrap();
        bh.push(out);
      })
    });
  }
}

criterion_group!(bench, criterion_benchmark);
criterion_main!(bench);