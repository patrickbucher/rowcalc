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
                write!(f, ">> until {:>6} up to {:>5}m", format_hms(time), dist)
            }
            Phase::Resting { time } => write!(f, "|| until {:>6}", format_hms(time)),
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
        if stints_finished > breaks_taken && breaks_taken < stints - 1 {
            elapsed_time = elapsed_time.saturating_add(pause);
            phases.push(Phase::Resting { time: elapsed_time });
            breaks_taken += 1;
        }
    }
    phases
}

pub fn calc_500m_split_time(dist: f32, time: Duration) -> Duration {
    let secs = time.as_secs_f32();
    let secs_per_meter = secs / dist;
    Duration::from_secs_f32(secs_per_meter * 500.0)
}

pub fn format_hms(time: &Duration) -> String {
    let secs = time.as_secs();
    let hours = secs / 3600;
    let secs = secs % 3600;
    let mins = secs / 60;
    let secs = secs % 60;
    let mut buf = String::new();
    if hours > 0 {
        buf.push_str(&format!("{hours}h"));
    }
    if mins > 0 {
        buf.push_str(&format!("{mins}m"));
    }
    buf.push_str(&format!("{secs:02}s"));
    buf
}
