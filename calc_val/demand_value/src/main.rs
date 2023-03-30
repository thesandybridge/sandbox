use std::fs;
use clap::Parser;
use anyhow::Result;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of file to generate or append.
    #[arg()]
    path: String,

    /// Number of jobs available.
    #[arg(short, long, default_value_t = 10000)]
    jobs: usize,

    /// Total experience.
    #[arg(short, long, default_value_t = 5000)]
    time: usize,

    /// Average salary.
    #[arg(short, long, default_value_t = 100000)]
    salary: usize,
}

fn value(demand: usize, curve: usize) -> usize {
    return demand * curve;
}

fn demand(jobs: usize, supply: usize) -> usize {
    return jobs / supply;
}

fn average_time(times: Vec<usize>) -> usize {
    let sum: usize = times.iter().sum();
    let average = sum / times.len();
    return average;
}

fn calc_curve(time: usize, average: usize) -> usize{
    return time / average;
}

fn percentage(value: usize, n: usize, sub: Option<bool>) -> usize {
    if let Some(t) = sub {

        if t {
            return value - (value * n / 100);
        }

        return value + (value * n / 100);
    }

    return value + (value * n / 100);
}

fn salary_potential(average: usize, value: usize) -> usize {
    let base;
    match value {
        0..=1 => {
            base = percentage(average, 15, Some(true));
        },
        a if a == 1 => {
            base = average;
        },
        _ => {
            base = percentage(average, value, None);
        }

    }
    return base;
}

fn main() -> Result<()> {
    let args = Args::parse();
    let jobs = args.jobs;
    let hours = args.time;
    let salary = args.salary;
    let path = args.path;

    let file = fs::read_to_string(path).expect("Missing file");

    let times = file
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let supply = times.len();

    let average = average_time(times);
    let dem = demand(jobs, supply);
    let curve = calc_curve(hours, average);
    let val = value(dem, curve);

    let potential = salary_potential(salary, val);

    println!("Supply: {}", supply);
    println!("Jobs: {}", jobs);
    println!("Hours: {}", hours);
    println!("Value: {}", val);
    println!("==========");
    println!("Salary Potential: {}", potential);

    return Ok(());
}
