use std::fmt::{self, Display, Formatter};
use std::time::Duration;

pub enum Phase {
    Rowing { dist: u32, time: Duration },
    Resting { time: Duration },
}

impl Display for Phase {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Phase::Rowing { dist, time } => {
                write!(f, "row up to {}m until {}", dist, fmt_mins_secs(time))
            }
            Phase::Resting { time } => write!(f, "break until {}", fmt_mins_secs(time)),
        }
    }
}

fn fmt_mins_secs(time: &Duration) -> String {
    let (mins, secs) = secs_to_mins_secs(time);
    if mins > 0 {
        format!("{mins}m{secs}s")
    } else {
        format!("{secs}s")
    }
}

fn secs_to_mins_secs(time: &Duration) -> (u64, u64) {
    let secs = time.as_secs();
    let mins = secs / 60;
    let secs = secs % 60;
    (mins, secs)
}
pub fn calc_split_time(dist: u32, secs: u32, size: f32) -> (u32, f32) {
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
