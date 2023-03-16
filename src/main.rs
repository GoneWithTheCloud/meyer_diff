use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

use crate::core::{MeyerDiff, MeyerDiffLineSpace};

pub mod core;

fn main() {
    for i in 1..=10 {
        for j in 1..=10 {
            compute(format!("{}.txt", i), format!("{}.txt", j));
        }
    }
}

fn compute(a: String, b: String) {
    let start = Instant::now();
    let f1 = File::open(a);
    let reader1 = BufReader::new(f1.unwrap());
    let mut v1: Vec<String> = Vec::new();

    for line in reader1.lines() {
        v1.push(line.unwrap());
    }
    let f2 = File::open(b);
    let reader2 = BufReader::new(f2.unwrap());
    let mut v2: Vec<String> = Vec::new();

    for line in reader2.lines() {
        v2.push(line.unwrap());
    }

    let meyer_diff = MeyerDiffLineSpace::new();
    let changes = meyer_diff.compute_diff(&v1, &v2);
    let mut modify = 0;
    for c in changes {
        modify += c.end_original() - c.start_original() + c.end_revised() - c.start_revised();
    }

    let sim = 1.0f32 - (modify as f32 + 0.0) / ((v1.len() + v2.len()) as f32 + 1e-8);
    println!("sim={} a={} b={} time={}", sim, v1.len(), v2.len(),start.elapsed().as_millis());
}
