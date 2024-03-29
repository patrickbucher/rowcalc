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

    /// Break duration in seconds
    #[arg(short, long)]
    breaks: u32,

    /// Split up session into stints (with breaks in between)
    #[arg(short, long)]
    stints: u32,
}

fn main() {
    let args = Args::parse();
    println!(
        "dist={}, time={}, breaks={}, stints={}",
        args.dist, args.time, args.breaks, args.stints
    );

    let total_breaks = (args.stints - 1) * args.breaks;
    let rowing_time = args.time * 60 - total_breaks;
    let n_500_splits = args.dist as f32 / 500.0;
    let split_time_secs = rowing_time as f32 / n_500_splits;
    let split_mins = split_time_secs as u32 / 60;
    let split_secs = split_time_secs - (split_mins * 60) as f32;

    println!("{}m{}s", split_mins, split_secs);
}
