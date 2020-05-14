use criterion::{black_box, criterion_group, criterion_main, Criterion};

use rand::seq::SliceRandom;
use rustweb::contrib::list::diff::DiffCallback;

pub struct Cb;
impl DiffCallback<(usize, &u32), (usize, &u32)> for Cb {
    fn inserted(&mut self, new: (usize, &u32)) {}
    fn removed(&mut self, old: (usize, &u32)) {}
    fn unchanged(&mut self, old: (usize, &u32), new: (usize, &u32)) {}
    fn moved(&mut self, old: (usize, &u32), new: (usize, &u32)) {}
}

fn diff(list1: &[u32], list2: &[u32]) {
    let mut cb = Cb;
    rustweb::contrib::list::diff::diff_by_key(
        list1.iter().enumerate(),
        |x| &x.1,
        list2.iter().enumerate(),
        |x| &x.1,
        &mut cb,
    );
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let list1 = (1..100000).collect::<Vec<_>>();
    let mut list2 = list1.clone();
    list2.shuffle(&mut rng);
    println!("{:?}", list1);
    println!("{:?}", list2);
    c.bench_function("diff 10000", |b| b.iter(|| diff(&list1, &list2)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
