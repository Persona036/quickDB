use criterion::{black_box, criterion_group, criterion_main, Criterion};
use db::{handle_command, load_from_file, save_to_file};
use std::collections::BTreeMap; // Import necessary functions

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut tables = BTreeMap::new();
    handle_command("create benchmark_table", &mut tables);

    c.bench_function("insert 1000 rows", |b| {
        b.iter(|| {
            for i in 0..1000 {
                handle_command(&format!("insert benchmark_table col {}", i), &mut tables);
            }
        })
    });

    let mut group = c.benchmark_group("select all rows");
    group.measurement_time(std::time::Duration::from_secs(60)); // Increase measurement time
    group.bench_function("select all rows", |b| {
        b.iter(|| {
            handle_command(black_box("select benchmark_table col"), &mut tables);
        })
    });
    group.finish();

    c.bench_function("save table", |b| {
        b.iter(|| {
            save_to_file(&tables, black_box("benchmark_db.json")).expect("Failed to save table");
        })
    });

    c.bench_function("load table", |b| {
        b.iter(|| {
            let loaded_tables =
                load_from_file(black_box("benchmark_db.json")).expect("Failed to load table");
            black_box(loaded_tables); // To avoid compiler optimizations
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

