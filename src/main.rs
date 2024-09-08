use clap::Parser;
use duration_str::parse;
use rowcalc::{calc_500m_split_time, calc_phases, format_hms};
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(version, about)]
/// Calculate 500m split time for rowing sessions.
struct Args {
    /// Distance in meters
    #[arg(short, long)]
    dist: f32,

    /// Time as duration string (e.g. 1h, 45m, 42m30s)
    #[arg(short, long)]
    time: String,

    /// Pause as duration string (e.g. 30s)
    #[arg(short, long)]
    pause: Option<String>,

    /// Breaks as number (e.g. 3)
    #[arg(short, long)]
    breaks: Option<u32>,
}

fn main() {
    let args = Args::parse();
    let dist = args.dist;
    let time = parse(&args.time).expect("time indication");
    let pause = parse(args.pause.unwrap_or("0s".into())).unwrap_or(Duration::new(0, 0));
    let breaks = args.breaks.unwrap_or(0);
    let stints = (breaks + 1) as usize;

    let total_break_time = pause.saturating_mul(breaks);
    let total_rowing_time = time.saturating_sub(total_break_time);
    let split_time = calc_500m_split_time(dist, time);
    let split_time_msg = format_hms(&split_time);
    let velocity = dist / total_rowing_time.as_secs_f32();

    let break_msg = if breaks > 0 {
        format!("with {breaks} breaks of {pause:?}")
    } else {
        String::from("without any breaks")
    };
    println!(
        "To row {}m in {} {} a 500m split time of {} and a velocity of {:.2}m/s is needed.",
        dist, args.time, break_msg, split_time_msg, velocity
    );

    let stint_dist: f32 = dist / stints as f32;
    let stint_time: f32 = total_rowing_time.as_secs_f32() / stints as f32;

    let split_dist: f32 = 500_f32;
    let split_time = Duration::from_secs((stint_time / (stint_dist / split_dist)).round() as u64);

    let phases = calc_phases(dist, split_dist, time, split_time, stints, pause);
    for (i, phase) in phases.iter().enumerate() {
        println!("{:2}) {}", i + 1, phase);
    }
}
