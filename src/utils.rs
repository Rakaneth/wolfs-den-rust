pub fn clamp<T: PartialOrd + Copy> (val: T, low: T, high: T) -> T {
    match &val {
        x if x < &low => low,
        x if x > &high => high,
        _ => val
    }
}

pub fn between<T: PartialOrd + Copy> (val: T, low: T, high: T) -> bool {
    clamp(val, low, high) == val
}