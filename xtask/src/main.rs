use std::collections::BTreeSet;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::Deserialize;

const BENCHMARK_GROUP: &str = "Pangui";

#[derive(Deserialize)]
struct BenchmarkMetadata {
    function_id: String,
    throughput: Option<Throughput>,
}

#[derive(Deserialize)]
enum Throughput {
    Bytes(u64),
    Elements(u64),
}

#[derive(Deserialize)]
struct Estimates {
    slope: Option<Estimate>,
    mean: Estimate,
}

#[derive(Deserialize)]
struct Estimate {
    confidence_interval: ConfidenceInterval,
    point_estimate: f64,
}

#[derive(Deserialize)]
struct ConfidenceInterval {
    lower_bound: f64,
    upper_bound: f64,
}

struct BenchmarkResult {
    name: String,
    nodes: u64,
    estimate_ns: f64,
    lower_ns: f64,
    upper_ns: f64,
}

struct Options {
    output: PathBuf,
    run_benchmarks: bool,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let repository = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask must be located directly under the repository root");
    let options = parse_options(repository)?;
    let criterion_group = repository.join("target/criterion").join(BENCHMARK_GROUP);

    if options.run_benchmarks {
        if criterion_group.exists() {
            fs::remove_dir_all(&criterion_group)?;
        }
        run_benchmarks(repository)?;
    }

    let results = read_results(&criterion_group)?;
    let skipped = find_skipped_fixtures(repository, &results)?;
    let report = render_report(repository, &results, &skipped);

    if let Some(parent) = options.output.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&options.output, report)?;
    println!("Wrote {}", options.output.display());
    Ok(())
}

fn parse_options(repository: &Path) -> Result<Options, Box<dyn Error>> {
    let mut output = repository.join("benches/pangui-results.md");
    let mut run_benchmarks = true;
    let mut arguments = env::args_os().skip(1);

    while let Some(argument) = arguments.next() {
        match argument.to_str() {
            Some("--no-bench") => run_benchmarks = false,
            Some("--output") => {
                let path = arguments.next().ok_or("--output requires a path")?;
                output = absolute_path(repository, path);
            }
            Some("--help" | "-h") => {
                println!(
                    "Usage: cargo pangui-report [--no-bench] [--output PATH]\n\n\
                     Runs the Pangui benchmarks and writes a Markdown report.\n\
                     --no-bench     Generate from existing Criterion data\n\
                     --output PATH  Output path (default: benches/pangui-results.md)"
                );
                std::process::exit(0);
            }
            _ => return Err(format!("unknown argument `{}`", argument.to_string_lossy()).into()),
        }
    }

    Ok(Options { output, run_benchmarks })
}

fn absolute_path(repository: &Path, path: OsString) -> PathBuf {
    let path = PathBuf::from(path);
    if path.is_absolute() {
        path
    } else {
        repository.join(path)
    }
}

fn run_benchmarks(repository: &Path) -> Result<(), Box<dyn Error>> {
    let cargo = env::var_os("CARGO").unwrap_or_else(|| OsString::from("cargo"));
    let status = Command::new(cargo).current_dir(repository).args(["bench", "--bench", "pangui"]).status()?;

    if !status.success() {
        return Err(format!("Pangui benchmark failed with {status}").into());
    }
    Ok(())
}

fn read_results(group_directory: &Path) -> Result<Vec<BenchmarkResult>, Box<dyn Error>> {
    if !group_directory.is_dir() {
        return Err(format!("Criterion results not found at {}", group_directory.display()).into());
    }

    let mut results = Vec::new();
    for entry in fs::read_dir(group_directory)? {
        let benchmark_directory = entry?.path().join("new");
        let metadata_path = benchmark_directory.join("benchmark.json");
        let estimates_path = benchmark_directory.join("estimates.json");
        if !metadata_path.is_file() || !estimates_path.is_file() {
            continue;
        }

        let metadata: BenchmarkMetadata = serde_json::from_slice(&fs::read(metadata_path)?)?;
        let estimates: Estimates = serde_json::from_slice(&fs::read(estimates_path)?)?;
        let estimate = estimates.slope.as_ref().unwrap_or(&estimates.mean);
        let nodes = match metadata.throughput {
            Some(Throughput::Elements(elements)) => elements,
            Some(Throughput::Bytes(bytes)) => {
                return Err(format!("benchmark `{}` reports byte throughput ({bytes})", metadata.function_id).into());
            }
            None => return Err(format!("benchmark `{}` has no node throughput", metadata.function_id).into()),
        };

        results.push(BenchmarkResult {
            name: metadata.function_id,
            nodes,
            estimate_ns: estimate.point_estimate,
            lower_ns: estimate.confidence_interval.lower_bound,
            upper_ns: estimate.confidence_interval.upper_bound,
        });
    }

    results.sort_by(|left, right| left.name.cmp(&right.name));
    if results.is_empty() {
        return Err(format!("no benchmark estimates found in {}", group_directory.display()).into());
    }
    Ok(results)
}

fn find_skipped_fixtures(repository: &Path, results: &[BenchmarkResult]) -> Result<Vec<String>, Box<dyn Error>> {
    let completed: BTreeSet<_> = results.iter().map(|result| result.name.as_str()).collect();
    let mut skipped = Vec::new();

    for entry in fs::read_dir(repository.join("benches/pangui"))? {
        let path = entry?.path();
        if path.extension().is_some_and(|extension| extension == "txt") {
            let name = path.file_stem().unwrap().to_string_lossy();
            if !completed.contains(name.as_ref()) {
                skipped.push(name.into_owned());
            }
        }
    }

    skipped.sort();
    Ok(skipped)
}

fn render_report(repository: &Path, results: &[BenchmarkResult], skipped: &[String]) -> String {
    let commit = command_output(repository, "git", &["rev-parse", "--short", "HEAD"]);
    let rust = command_output(repository, "rustc", &["--version"]);
    let mut report = format!(
        "# Pangui Benchmark Results\n\n\
         ## Environment\n\n\
         - Commit: `{commit}`\n\
         - Platform: {} {}\n\
         - Rust: {rust}\n\
         - Command: `cargo pangui-report`\n\n\
         Each fixture is parsed once, a fresh ECS tree is constructed outside the timed routine, and a full layout is measured. Times are Criterion slope point estimates with 95% confidence intervals.\n\n\
         ## Results\n\n\
         | Benchmark | Nodes | Time | 95% confidence interval | Throughput |\n\
         | --- | ---: | ---: | ---: | ---: |\n",
        env::consts::OS,
        env::consts::ARCH,
    );

    for result in results {
        let (time, lower, upper) = format_duration_range(result.estimate_ns, result.lower_ns, result.upper_ns);
        let throughput = result.nodes as f64 * 1_000.0 / result.estimate_ns;
        report.push_str(&format!(
            "| `{}` | {} | {time} | {lower}-{upper} | {throughput:.4} Melem/s |\n",
            result.name,
            format_integer(result.nodes),
        ));
    }

    if !skipped.is_empty() {
        report.push_str("\n## Skipped Fixtures\n\n");
        for fixture in skipped {
            report.push_str(&format!("- `{fixture}.txt` (unsupported or produced no Criterion result)\n"));
        }
    }

    report
}

fn command_output(repository: &Path, program: &str, arguments: &[&str]) -> String {
    Command::new(program)
        .current_dir(repository)
        .args(arguments)
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .filter(|output| !output.is_empty())
        .unwrap_or_else(|| "unknown".to_string())
}

fn format_duration_range(point: f64, lower: f64, upper: f64) -> (String, String, String) {
    let (divisor, suffix, precision) = if point >= 1_000_000_000.0 {
        (1_000_000_000.0, "s", 3)
    } else if point >= 1_000_000.0 {
        (1_000_000.0, "ms", if point >= 100_000_000.0 { 2 } else { 4 })
    } else if point >= 1_000.0 {
        (1_000.0, "us", 2)
    } else {
        (1.0, "ns", 2)
    };

    (
        format!("{:.*} {suffix}", precision, point / divisor),
        format!("{:.*}", precision, lower / divisor),
        format!("{:.*} {suffix}", precision, upper / divisor),
    )
}

fn format_integer(value: u64) -> String {
    let digits = value.to_string();
    let mut output = String::with_capacity(digits.len() + digits.len() / 3);
    for (index, character) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index).is_multiple_of(3) {
            output.push(',');
        }
        output.push(character);
    }
    output
}

#[cfg(test)]
mod tests {
    use super::{format_duration_range, format_integer};

    #[test]
    fn formats_duration_range_using_point_unit() {
        assert_eq!(
            format_duration_range(1_796_047.0, 1_766_058.0, 1_833_658.0),
            ("1.7960 ms".to_string(), "1.7661".to_string(), "1.8337 ms".to_string())
        );
    }

    #[test]
    fn formats_integer_with_grouping() {
        assert_eq!(format_integer(101_111), "101,111");
    }
}
