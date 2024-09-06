use clap::Parser;
use duration_str::parse;
use rowcalc::{calc_split_time, Phase};
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(version, about)]
/// Calculate 500m split time for rowing sessions.
struct Args {
    /// Distance in meters
    #[arg(short, long)]
    dist: u32,

    /// Time as duration string (e.g. 1h, 45m, 42m30s)
    #[arg(short, long)]
    time: String,

    /// Pause as duration string (e.g. 30s)
    #[arg(short, long)]
    pause: Option<String>,

    /// Breaks as number (e.g. 3)
    #[arg(short, long)]
    breaks: Option<u32>,
    // TODO: specify split time (e.g. 5m) or split distance (e.g. 1000m)
}

fn main() {
    let args = Args::parse();
    let dist = args.dist;
    let time = parse(args.time).expect("time indication");
    let pause = parse(args.pause.unwrap_or("0s".into())).unwrap_or(Duration::new(0, 0));
    let breaks = args.breaks.unwrap_or(0);
    let stints = breaks + 1;
    println!(
        "row {}m in {:?} with {} breaks of {:?}",
        dist, time, breaks, pause
    );

    let total_break_time = pause.saturating_mul(breaks);
    println!("total pause time: {total_break_time:?}");

    let total_rowing_time = time.saturating_sub(total_break_time);
    println!("total rowing time: {total_rowing_time:?}");

    let stint_rowing_time = total_rowing_time.div_f32(stints as f32);
    println!("stint rowing time: {stint_rowing_time:?}");

    let velocity = dist as f32 / total_rowing_time.as_secs_f32(); // m/s
    println!("velocity: {velocity:?}m/s");

    let split_time = calc_split_time(dist, time.as_secs() as u32, 500_f32);
    println!("500m split time: {}m{}s", split_time.0, split_time.1);

    println!();

    let stint_dist: f32 = dist as f32 / stints as f32;
    let stint_time: f32 = total_rowing_time.as_secs_f32() / stints as f32;

    let split_dist: f32 = 500_f32; // TODO: fill with argument (to be defined) from above
    let split_time = Duration::from_secs((stint_time / (stint_dist / split_dist)).round() as u64);
    println!("split dist: {}m", split_dist);
    println!("split time: {:?}", split_time);

    let mut phases: Vec<Phase> = Vec::new();
    let mut elapsed_dist: f32 = 0.0;
    let mut elapsed_time = Duration::new(0, 0);
    let mut breaks_taken: usize = 0;
    while elapsed_dist < dist as f32 {
        elapsed_dist += split_dist;
        elapsed_time = elapsed_time.saturating_add(split_time);
        phases.push(Phase::Rowing {
            time: elapsed_time,
            dist: elapsed_dist as u32,
        });
        let stints_finished = ((elapsed_dist / dist as f32) * stints as f32).floor();
        if stints_finished as u32 == stints {
            break;
        }
        if stints_finished as usize > breaks_taken {
            elapsed_time = elapsed_time.saturating_add(pause);
            phases.push(Phase::Resting { time: elapsed_time });
            breaks_taken += 1;
        }
    }
    for phase in phases {
        println!("{}", phase);
    }
}
