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

pub fn calc_phases(
    dist: f32,
    split_dist: f32,
    time: Duration,
    split_time: Duration,
    stints: usize,
    pause: Duration,
) -> Vec<Phase> {
    let mut phases: Vec<Phase> = Vec::new();
    let mut elapsed_dist: f32 = 0.0;
    let mut elapsed_time = Duration::new(0, 0);
    let mut breaks_taken: usize = 0;
    while elapsed_dist < dist {
        let dist_left = dist - elapsed_dist;
        let time_left = time.saturating_sub(elapsed_time);
        let (split_dist, split_time) = if dist_left < split_dist {
            (dist_left, time_left)
        } else {
            (split_dist, split_time)
        };
        elapsed_dist += split_dist;
        elapsed_time = elapsed_time.saturating_add(split_time);
        phases.push(Phase::Rowing {
            time: elapsed_time,
            dist: elapsed_dist as u32,
        });
        let stints_finished = ((elapsed_dist / dist) * stints as f32).floor() as usize;
        if stints_finished == stints {
            break;
        }
        if stints_finished > breaks_taken {
            elapsed_time = elapsed_time.saturating_add(pause);
            phases.push(Phase::Resting { time: elapsed_time });
            breaks_taken += 1;
        }
    }
    phases
}

fn fmt_mins_secs(time: &Duration) -> String {
    let (mins, secs) = secs_to_mins_secs(time);
    if mins > 0 {
        format!("{mins}m{secs}s")
    } else {
        format!("{secs}s")
    }
}

pub fn calc_split_time(dist: u32, secs: u32, size: f32) -> (u32, f32) {
    let n_500_splits = dist as f32 / size;
    let split_time_secs = secs as f32 / n_500_splits;
    let split_mins = split_time_secs as u32 / 60;
    let split_secs = split_time_secs - (split_mins * 60) as f32;
    (split_mins, round(split_secs, 1.0))
}

fn secs_to_mins_secs(time: &Duration) -> (u64, u64) {
    let secs = time.as_secs();
    let mins = secs / 60;
    let secs = secs % 60;
    (mins, secs)
}

fn round(value: f32, granularity: f32) -> f32 {
    let stretch = 1.0 / granularity;
    let scaled = value * stretch;
    let rounded = scaled.round();
    rounded / stretch
}
