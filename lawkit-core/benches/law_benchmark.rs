use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lawkit_core::{
    common::input::parse_input_auto,
    laws::{
        benford::BenfordResult, normal::NormalResult, pareto::ParetoResult, poisson::PoissonResult,
        zipf::ZipfResult,
    },
};

fn bench_benford_analysis(c: &mut Criterion) {
    let test_data = vec![
        123.45, 234.56, 345.67, 456.78, 567.89, 678.90, 789.01, 890.12, 901.23, 112.34, 223.45,
        334.56, 445.67, 556.78, 667.89, 778.90, 889.01, 990.12, 101.23, 212.34, 323.45, 434.56,
        545.67, 656.78, 767.89, 878.90, 989.01, 190.12, 281.23, 372.34, 463.45, 554.56,
    ];

    c.bench_function("benford_analysis", |b| {
        b.iter(|| {
            let result = BenfordResult::new("benchmark".to_string(), black_box(&test_data));
            black_box(result)
        })
    });
}

fn bench_pareto_analysis(c: &mut Criterion) {
    let test_data = vec![
        100.0, 200.0, 150.0, 300.0, 250.0, 400.0, 350.0, 500.0, 450.0, 600.0, 550.0, 700.0, 650.0,
        800.0, 750.0, 900.0, 850.0, 1000.0, 950.0, 1100.0, 1050.0, 1200.0, 1150.0, 1300.0, 1250.0,
        1400.0, 1350.0, 1500.0, 1450.0, 1600.0, 1550.0, 1700.0,
    ];

    c.bench_function("pareto_analysis", |b| {
        b.iter(|| {
            let result = ParetoResult::new("benchmark".to_string(), black_box(&test_data));
            black_box(result)
        })
    });
}

fn bench_zipf_analysis(c: &mut Criterion) {
    let test_data = vec![
        1000.0, 500.0, 333.0, 250.0, 200.0, 166.0, 142.0, 125.0, 111.0, 100.0, 90.0, 83.0, 76.0,
        71.0, 66.0, 62.0, 58.0, 55.0, 52.0, 50.0, 47.0, 45.0, 43.0, 41.0, 40.0, 38.0, 37.0, 35.0,
        34.0, 33.0, 32.0, 31.0,
    ];

    c.bench_function("zipf_analysis", |b| {
        b.iter(|| {
            let result = ZipfResult::new("benchmark".to_string(), black_box(&test_data));
            black_box(result)
        })
    });
}

fn bench_normal_analysis(c: &mut Criterion) {
    let test_data = vec![
        10.0, 12.0, 11.0, 13.0, 9.0, 14.0, 8.0, 15.0, 7.0, 16.0, 6.0, 17.0, 5.0, 18.0, 4.0, 19.0,
        3.0, 20.0, 2.0, 21.0, 1.0, 22.0, 0.0, 23.0, 11.5, 12.5, 10.5, 13.5, 9.5, 14.5, 8.5, 15.5,
    ];

    c.bench_function("normal_analysis", |b| {
        b.iter(|| {
            let result = NormalResult::new("benchmark".to_string(), black_box(&test_data));
            black_box(result)
        })
    });
}

fn bench_poisson_analysis(c: &mut Criterion) {
    let test_data = vec![
        0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 1.0, 2.0, 0.0, 3.0, 1.0, 4.0, 2.0, 5.0, 3.0, 1.0, 0.0, 2.0,
        4.0, 3.0, 1.0, 5.0, 2.0, 0.0, 3.0, 1.0, 4.0, 2.0, 5.0, 0.0, 3.0, 1.0,
    ];

    c.bench_function("poisson_analysis", |b| {
        b.iter(|| {
            let result = PoissonResult::new("benchmark".to_string(), black_box(&test_data));
            black_box(result)
        })
    });
}

fn bench_input_parsing(c: &mut Criterion) {
    let test_input = "123.45,234.56,345.67,456.78,567.89,678.90,789.01,890.12";

    c.bench_function("input_parsing", |b| {
        b.iter(|| {
            let result = parse_input_auto(black_box(test_input));
            black_box(result)
        })
    });
}

criterion_group!(
    benches,
    bench_benford_analysis,
    bench_pareto_analysis,
    bench_zipf_analysis,
    bench_normal_analysis,
    bench_poisson_analysis,
    bench_input_parsing
);
criterion_main!(benches);
