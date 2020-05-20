use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use sqr_heap::sqr_heap::SqrHeap;
use std::{collections::BinaryHeap, time::Duration};

pub fn criterion_benchmark(c: &mut Criterion) {
  let range = (10..30).flat_map(|p| {
    vec![
      1 << p,
      (1 << p) + (1 << (p - 1)),
      (1 << p) + (1 << (p - 1)) + (1 << (p - 2)),
      (1 << p) + (1 << (p - 1)) + (1 << (p - 2)) + (1 << (p - 3)),
    ]
  });
  let mut group = c.benchmark_group("pop_push_pair");
  for l in range {
    let mut sh = SqrHeap::new();
    let mut bh = BinaryHeap::new();
    for i in 0..l {
      sh.push(i);
      bh.push(i);
    }
    group.bench_with_input(BenchmarkId::new("Sqr Heap", l), &l, |b, _| {
      b.iter(|| {
        let out = black_box(sh.pop().unwrap());
        sh.push(out);
      })
    });
    group.bench_with_input(BenchmarkId::new("Bin Heap", l), &l, |b, _| {
      b.iter(|| {
        let out = black_box(bh.pop().unwrap());
        bh.push(out);
      })
    });
  }
}

criterion_group! {
  name = bench;
  config = Criterion::default().warm_up_time(Duration::from_secs(5));
  targets = criterion_benchmark,
}
criterion_main!(bench);
