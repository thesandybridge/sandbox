use clap::Parser;
use anyhow::Result;
use sbx_common::{add_percent, sub_percent, generate_nordis_vec};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of available people available.
    #[arg(short, long, default_value_t = 10000)]
    people: usize,

    /// Number of jobs available.
    #[arg(short, long, default_value_t = 10000)]
    jobs: usize,

    /// Total experience.
    #[arg(short, long, default_value_t = 5000)]
    time: usize,

    /// Average salary.
    #[arg(short, long, default_value_t = 100000)]
    salary: usize,

    /// Enable verbose output
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

#[macro_export]
macro_rules! debug {
    ($debug:ident, $message:literal, $($value:expr),* ) => {
        {
           if $debug {
               print!("DEBUG: ");
               println!($message, $($value),*);
           }
        }
    };
}

fn value(demand: usize, curve: usize, debug: bool) -> usize {
    debug!(debug, "value = {} * {}", demand, curve);
    return demand * curve;

}

fn demand(jobs: usize, supply: usize, debug: bool) -> usize {
    debug!(debug, "demand = {} / {}", jobs, supply);
    return jobs / supply;
}

fn average_time(times: &Vec<f64>, debug: bool) -> usize {
    let sum: f64 = times.iter().sum();
    debug!(debug, "average = {} / {}", sum, times.len());
    let average = sum as usize / times.len();
    return average;
}

fn calc_curve(time: usize, average: usize, debug: bool) -> usize{
    debug!(debug, "curve = {} / {}", time, average);
    return time / average;
}

fn salary_potential(average: usize, value: usize, debug: bool) -> usize {
    let base;
    match value {
        0..=1 => {
            base = sub_percent(average, 15);
            debug!(debug, "value is less than 1 - salary: {}", base);
        },
        a if a == 1 => {
            base = average;
            debug!(debug, "value is equal to 1 - salary: {}", base);
        },
        _ => {
            base = add_percent(average, value);
            debug!(debug, "value is greater than 1 - salary: {}", base);
        }
    }
    return base;
}

fn main() -> Result<()> {
    let args = Args::parse();
    let jobs = args.jobs;
    let hours = args.time;
    let salary = args.salary;
    let supply = args.people;
    let debug = args.verbose;

    let times = generate_nordis_vec(supply, 5000.0, 5000.0, 0.0, 10000.0);
    let average = average_time(&times, debug);
    let dem = demand(jobs, supply, debug);
    let curve = calc_curve(hours, average, debug);
    let val = value(dem, curve, debug);

    let potential = salary_potential(salary, val, debug);

    println!("Supply: {}", supply);
    println!("Jobs: {}", jobs);
    println!("Hours: {}", hours);
    println!("Value: {}", val);
    println!("==========");
    println!("Salary Potential: {}", potential);

    return Ok(());
}
