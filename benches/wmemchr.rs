use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;
use wmemchr_benches::wmemchr;

const SMALL: usize = 256;
const MEDIUM: usize = 66536;

const fn middle(n: usize) -> usize {
    n * 3 / 4
}

fn bench_small_no_null(c: &mut Criterion) {
    let mut rng = rand::thread_rng();

    // exclude NUL byte from 1..256 range
    let small_no_null: Vec<u16> = (0..SMALL).map(|_| rng.gen_range(1, 256) as u16).collect();

    let mut group = c.benchmark_group("small slice      [no null in slice]");
    group.bench_function("iter::position", |b| {
        b.iter(|| small_no_null.iter().position(|&c| c == 0))
    });
    group.bench_function("wmemchr", |b| b.iter(|| wmemchr(0, &small_no_null)));
    group.finish();
}

fn bench_small_null_middle(c: &mut Criterion) {
    let mut rng = rand::thread_rng();

    // exclude NUL byte from 1..256 range
    let mut small_nul_middle: Vec<u16> = (0..SMALL).map(|_| rng.gen_range(1, 256) as u16).collect();
    small_nul_middle[middle(SMALL)] = 0;

    let mut group = c.benchmark_group("small slice      [null in the middle of slice]");
    group.bench_function("iter::position", |b| {
        b.iter(|| small_nul_middle.iter().position(|&c| c == 0))
    });
    group.bench_function("wmemchr", |b| b.iter(|| wmemchr(0, &small_nul_middle)));
    group.finish();
}

fn bench_small_null_end(c: &mut Criterion) {
    let mut rng = rand::thread_rng();

    // exclude NUL byte from 1..256 range
    let mut small_nul_end: Vec<u16> = (0..SMALL).map(|_| rng.gen_range(1, 256) as u16).collect();
    small_nul_end[SMALL - 1] = 0;

    let mut group = c.benchmark_group("small slice      [null at the end of slice]");
    group.bench_function("iter::position", |b| {
        b.iter(|| small_nul_end.iter().position(|&c| c == 0))
    });
    group.bench_function("wmemchr", |b| b.iter(|| wmemchr(0, &small_nul_end)));
    group.finish();
}

fn bench_medium_no_null(c: &mut Criterion) {
    let mut rng = rand::thread_rng();

    // exclude NUL byte from 1..256 range
    let medium_no_null: Vec<u16> = (0..MEDIUM).map(|_| rng.gen_range(1, 256) as u16).collect();

    let mut group = c.benchmark_group("[medium slice    [no null in slice]");
    group.bench_function("iter::position", |b| {
        b.iter(|| medium_no_null.iter().position(|&c| c == 0))
    });
    group.bench_function("wmemchr", |b| b.iter(|| wmemchr(0, &medium_no_null)));
    group.finish();
}

fn bench_medium_null_middle(c: &mut Criterion) {
    let mut rng = rand::thread_rng();

    // exclude NUL byte from 1..256 range
    let mut medium_nul_middle: Vec<u16> =
        (0..MEDIUM).map(|_| rng.gen_range(1, 256) as u16).collect();
    medium_nul_middle[middle(MEDIUM)] = 0;

    let mut group = c.benchmark_group("medium slice     [null in the middle of slice]");
    group.bench_function("iter::position", |b| {
        b.iter(|| medium_nul_middle.iter().position(|&c| c == 0))
    });
    group.bench_function("wmemchr", |b| b.iter(|| wmemchr(0, &medium_nul_middle)));
    group.finish();
}

fn bench_medium_null_end(c: &mut Criterion) {
    let mut rng = rand::thread_rng();

    // exclude NUL byte from 1..256 range
    let mut medium_nul_end: Vec<u16> = (0..MEDIUM).map(|_| rng.gen_range(1, 256) as u16).collect();
    medium_nul_end[MEDIUM - 1] = 0;

    let mut group = c.benchmark_group("medium slice     [null at the end of slice]");
    group.bench_function("iter::position", |b| {
        b.iter(|| medium_nul_end.iter().position(|&c| c == 0))
    });
    group.bench_function("wmemchr", |b| b.iter(|| wmemchr(0, &medium_nul_end)));
    group.finish();
}

criterion_group!(
    benches,
    bench_small_no_null,
    bench_small_null_middle,
    bench_small_null_end,
    bench_medium_no_null,
    bench_medium_null_middle,
    bench_medium_null_end,
);
criterion_main!(benches);
