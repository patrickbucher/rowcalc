fn calc_stints(time: u32, dist: u32, stints: u32) -> Vec<(u32, u32, u32)> {
    let mut parts: Vec<(u32, u32, u32)> = Vec::new();
    let split_time = (time * 60) as f32 / (stints as f32);
    let split_dist = dist as f32 / (stints as f32);
    for i in 1..=stints {
        let time = i as f32 * split_time;
        let dist = i as f32 * split_dist;
        parts.push((i, dist.round() as u32, time.round() as u32));
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
