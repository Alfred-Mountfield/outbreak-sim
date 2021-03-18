use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput, BenchmarkId};

struct U32Bool {
    val: u32,
    active: bool,
}

struct U64Bool {
    val: u64,
    active: bool,
}

// TOOD Try nonmax
fn bench_u32_loop(c: &mut Criterion) {
    let mut group = c.benchmark_group("u32 Looping");
    for size in [100u32, 1_000u32, 10_000u32, 1_000_000u32, 10_000_000u32].iter() {
        let range_vec: Vec<u32> = (0..*size).collect();
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| range_vec.iter().for_each(|idx| { black_box(idx + 1); }));
        });
    }
    group.finish();
}

fn bench_u32_bool_loop(c: &mut Criterion) {
    let mut group = c.benchmark_group("u32 w/ Bool Looping");
    for size in [100u32, 1_000u32, 10_000u32, 1_000_000u32, 10_000_000u32].iter() {
        let range_vec: Vec<U32Bool> = (0..*size).map(|idx| U32Bool { val: idx, active: false }).collect();
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| range_vec.iter().for_each(|idx| { black_box(idx); }));
        });
    }
    group.finish();
}

fn bench_u32_option_loop(c: &mut Criterion) {
    let mut group = c.benchmark_group("u32 w/ Option Looping");
    for size in [100u32, 1_000u32, 10_000u32, 1_000_000u32, 10_000_000u32].iter() {
        let range_vec: Vec<Option<u32>> = (0..*size).map(|idx| Some(idx)).collect();
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| range_vec.iter().for_each(|idx| { black_box(idx); }));
        });
    }
    group.finish();
}

fn bench_u64_loop(c: &mut Criterion) {
    let mut group = c.benchmark_group("u64 Looping");
    for size in [100u64, 1_000u64, 10_000u64, 1_000_000u64, 10_000_000u64].iter() {
        let range_vec: Vec<u64> = (0..*size).collect();
        group.throughput(Throughput::Elements(*size));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| range_vec.iter().for_each(|idx| { black_box(idx); }));
        });
    }
    group.finish();
}

fn bench_u64_bool_loop(c: &mut Criterion) {
    let mut group = c.benchmark_group("u64 w/ Bool Looping");
    for size in [100u64, 1_000u64, 10_000u64, 1_000_000u64, 10_000_000u64].iter() {
        let range_vec: Vec<U64Bool> = (0..*size).map(|idx| U64Bool { val: idx, active: false }).collect();
        group.throughput(Throughput::Elements(*size));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| range_vec.iter().for_each(|idx| { black_box(idx); }));
        });
    }
    group.finish();
}

fn bench_u64_option_loop(c: &mut Criterion) {
    let mut group = c.benchmark_group("u64 w/ Option Looping");
    for size in [100u64, 1_000u64, 10_000u64, 1_000_000u64, 10_000_000u64].iter() {
        let range_vec: Vec<Option<u64>> = (0..*size).map(|idx| Some(idx)).collect();
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| range_vec.iter().for_each(|idx| { black_box(idx); }));
        });
    }
    group.finish();
}


criterion_group!(benches, bench_u32_loop, bench_u32_bool_loop, bench_u32_option_loop, bench_u64_loop, bench_u64_bool_loop, bench_u64_option_loop);
criterion_main!(benches);