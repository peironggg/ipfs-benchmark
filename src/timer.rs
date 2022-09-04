use std::time::{Duration, Instant};

pub fn measure_time<T, F: FnOnce() -> T>(f: F) -> (Duration, T) {
  let start = Instant::now();
  let result = f();
  let elapsed_duration = start.elapsed();

  (elapsed_duration, result)
}
