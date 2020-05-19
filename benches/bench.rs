use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use sqr_heap::sqr_heap::SqrHeap;
use std::{collections::BinaryHeap, time::Duration};

pub fn criterion_benchmark(c: &mut Criterion) {
  let range = (4..28).flat_map(|p| vec![1 << p, (1 << p) + 1 << (p-1)]);
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

criterion_group! {
  name = bench;
  config = Criterion::default().warm_up_time(Duration::from_secs(8));
  targets = criterion_benchmark,
}
criterion_main!(bench);
