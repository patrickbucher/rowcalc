use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
/// Calculate 500m split time for rowing sessions.
struct Args {
    /// Distance in meters
    #[arg(short, long)]
    dist: u32,

    /// Time in minutes
    #[arg(short, long)]
    time: u32,

    /// Report target goals by distance
    #[arg(short, long, default_value_t = 1000)]
    by_dist: u32,
}

fn main() {
    let args = Args::parse();
    let (split_mins_500m, split_secs_500m) = calc_split_time(args.dist, args.time * 60, 500.0);
    let steps = args.dist / args.by_dist;
    let secs_per_dist = (args.time as f32 * 60.0) / (args.dist as f32 / args.by_dist as f32);
    println!("{:>6}  {:>10} {:>10}", "split", "dist", "time");
    for i in 1..=steps {
        let dist = i * args.by_dist;
        let time = i as f32 * secs_per_dist;
        println!("{:>6}. {:>10} {:>10}s", i, dist, time.round());
    }
    println!("time per 500m: {}m{:02}s", split_mins_500m, split_secs_500m);
}

fn calc_split_time(dist: u32, secs: u32, size: f32) -> (u32, f32) {
    let n_500_splits = dist as f32 / size;
    let split_time_secs = secs as f32 / n_500_splits;
    let split_mins = split_time_secs as u32 / 60;
    let split_secs = split_time_secs - (split_mins * 60) as f32;
    (split_mins, round(split_secs, 1.0))
}

fn round(value: f32, granularity: f32) -> f32 {
    let stretch = 1.0 / granularity;
    let scaled = value * stretch;
    let rounded = scaled.round();
    rounded / stretch
}
