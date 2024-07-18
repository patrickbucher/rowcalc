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

    /// Report target goals by distance (in meters)
    #[arg(long)]
    by_dist: Option<u32>,

    /// Report target goals by time (in seconds)
    #[arg(long)]
    by_time: Option<u32>,
}

fn main() {
    let args = Args::parse();

    if args.by_dist.and(args.by_time).is_some() {
        panic!("report target either by --by-dist or --by-time, but not by both");
    } else if let Some(by_dist) = args.by_dist {
        let secs_per_dist = (args.time as f32 * 60.0) / (args.dist as f32 / by_dist as f32);
        let steps = args.dist / by_dist;

        // TODO: separate calculation from output
        println!("{:>6}  {:>10} {:>10}", "split", "dist", "time");
        for i in 1..=steps {
            let dist = i * by_dist;
            let time = i as f32 * secs_per_dist;
            println!("{:>6}. {:>10} {:>10}s", i, dist, time.round());
        }
    } else if let Some(by_time) = args.by_time {
        let dist_per_secs = (args.dist as f32) / (args.time as f32 * 60.0 / by_time as f32);
        let steps = args.time / (by_time / 60);

        // TODO: separate calculation from output
        println!("{:>6}  {:>10} {:>10}", "split", "dist", "time");
        for i in 1..=steps {
            let secs = i * by_time;
            let dist = i as f32 * dist_per_secs;
            println!("{:>6}. {:>10} {:>10}s", i, dist.round(), secs);
        }
    } else {
        panic!("report target either --by-dist or --by-time, but not by both");
    }

    let (split_mins_500m, split_secs_500m) = calc_split_time(args.dist, args.time * 60, 500.0);
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
