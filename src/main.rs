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

    let (split_mins_500m, split_secs_500m) = calc_split_time(args.dist, args.time * 60, 500.0);
    let parts: Vec<(u32, u32, u32)> = if args.by_dist.and(args.by_time).is_some() {
        panic!("report target either by --by-dist or --by-time, but not by both");
    } else if let Some(by_dist) = args.by_dist {
        let stints = args.dist / by_dist;
        calc_stints(args.time, args.dist, stints)
    } else if let Some(by_time) = args.by_time {
        let stints = args.time * 60 / by_time;
        calc_stints(args.time, args.dist, stints)
    } else {
        panic!("report target either --by-dist or --by-time, but not by both");
    };

    println!("time per 500m: {}m{:02}s", split_mins_500m, split_secs_500m);
    if !parts.is_empty() {
        println!("{:>6}  {:>10} {:>10}", "split", "dist", "time");
        for (i, dist, time) in parts {
            println!("{:>6}. {:>10} {:>10}s", i, dist, time);
        }
    }
}

fn calc_stints(time: u32, dist: u32, stints: u32) -> Vec<(u32, u32, u32)> {
    let mut parts: Vec<(u32, u32, u32)> = Vec::new();
    let split_time = time * 60 / stints;
    let split_dist = dist / stints;
    for i in 1..=stints {
        let time = i * split_time;
        let dist = i * split_dist;
        parts.push((i, dist, time));
    }
    parts
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
