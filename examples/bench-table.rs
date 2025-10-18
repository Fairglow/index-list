use comfy_table::{Cell, Row, Table};
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufReader;
use walkdir::WalkDir;

#[derive(Deserialize, Debug)]
struct Estimate {
    point_estimate: f64,
}

#[derive(Deserialize, Debug)]
struct Benchmark {
    mean: Estimate,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut versions = vec!["new"];

    if args.len() > 1 {
        versions.clear();
        if args.contains(&"--base".to_string()) {
            versions.push("base");
        }
        if args.contains(&"--new".to_string()) {
            versions.push("new");
        }
    }

    for version in versions {
        generate_table(version);
    }
}

fn generate_table(version: &str) {
    let mut results: HashMap<String, HashMap<String, f64>> = HashMap::new();

    for entry in WalkDir::new("target/criterion")
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_name().to_str() == Some("estimates.json") {
            let path = entry.path();
            if path.to_str().map_or(false, |s| s.contains(&format!("/{}/", version))) {
                if let Some(parent) = path.parent() {
                    if let Some(benchmark_dir) = parent.parent() {
                        if let Some(benchmark_name) = benchmark_dir.file_name() {
                            if let Some(benchmark_name_str) = benchmark_name.to_str() {
                                let file = File::open(path).unwrap();
                                let reader = BufReader::new(file);
                                let benchmark: Benchmark = serde_json::from_reader(reader).unwrap();

                                let parts: Vec<&str> = benchmark_name_str.split('-').collect();
                                if parts.len() >= 2 {
                                    let implementation = parts[0].to_string();
                                    let operation = parts[1..].join("-");

                                    results
                                        .entry(operation)
                                        .or_default()
                                        .insert(implementation, benchmark.mean.point_estimate);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if results.is_empty() {
        println!("No benchmark results found for '{}'. Run 'cargo bench' first.", version);
        return;
    }

    println!("\nResults for: {}", version);

    let mut operations: Vec<String> = results.keys().cloned().collect();
    operations.sort();

    let mut implementations: Vec<String> = results
        .values()
        .flat_map(|x| x.keys())
        .map(|s| s.to_string())
        .collect();
    implementations.sort();
    implementations.dedup();

    let mut table = Table::new();
    let mut header = vec![
        Cell::new("Benchmark"),
        Cell::new("Best Time"),
        Cell::new("Worst Time"),
    ];
    for impl_name in &implementations {
        header.push(Cell::new(impl_name));
    }
    table.set_header(header);

    for op in operations {
        let mut row = Row::new();
        row.add_cell(Cell::new(&op));

        let op_results = results.get(&op).unwrap();
        let best_time = op_results.values().cloned().fold(f64::INFINITY, f64::min);
        let worst_time = op_results.values().cloned().fold(f64::NEG_INFINITY, f64::max);

        row.add_cell(Cell::new(format_time(best_time)));
        row.add_cell(Cell::new(format_time(worst_time)));

        for impl_name in &implementations {
            if let Some(time) = op_results.get(impl_name) {
                let relative = time / best_time;
                row.add_cell(Cell::new(format!("{:.2}x", relative)));
            } else {
                row.add_cell(Cell::new("-"));
            }
        }
        table.add_row(row);
    }

    println!("{}", table);
}

fn format_time(time_ns: f64) -> String {
    if time_ns < 1_000.0 {
        format!("{:.2} ns", time_ns)
    } else if time_ns < 1_000_000.0 {
        format!("{:.2} µs", time_ns / 1_000.0)
    } else if time_ns < 1_000_000_000.0 {
        format!("{:.2} ms", time_ns / 1_000_000.0)
    } else {
        format!("{:.2} s", time_ns / 1_000_000_000.0)
    }
}
