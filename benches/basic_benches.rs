use std::f32::consts::PI;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

use topological_peak_detection::{
    find_homologies, 
    get_peaks,
};

fn basic_peaks(v: &Vec<f32>){
    let homologies = find_homologies(v).unwrap();
    let _x = get_peaks(&homologies);
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let tst_vec_sixty = (0..60)
        .map(|x| ((x as f32 / 10_f32) * PI).sin())
        .collect();

    let tst_vec_sixhundred = (0..600)
        .map(|x| ((x as f32 / 100_f32) * PI).sin())
        .collect();

    let tst_vec_6k = (0..6000)
        .map(|x| ((x as f32 / 1000_f32) * PI).sin())
        .collect();

    c.bench_function(
        "Sinuisoid /w 4 peaks, 60 elements", 
        |b| b.iter(|| basic_peaks(black_box(&tst_vec_sixty)))
    );
    
    c.bench_function(
        "Sinuisoid /w 4 peaks, 600 elements", 
        |b| b.iter(|| basic_peaks(black_box(&tst_vec_sixhundred)))
    );

    c.bench_function(
        "Sinuisoid /w 4 peaks, 6,000 elements", 
        |b| b.iter(|| basic_peaks(black_box(&tst_vec_6k)))
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);