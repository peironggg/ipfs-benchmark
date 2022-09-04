use crate::client::{BenchmarkClient, BenchmarkError};
use crate::response::body::AddFileResponseBody;
use std::time::{Duration};

#[derive(Debug)]
pub struct BenchmarkResult<T, E> {
  _max_time: Duration,
  _min_time: Duration,
  _avg_time: Duration,
  _successes: Vec<T>,
  _errors: Vec<E>,
}

pub struct Executor {
  client: BenchmarkClient,
  url_path: &'static str,
}

impl Executor {
  pub fn new(client: BenchmarkClient, url_path: &'static str) -> Executor {
    Executor {
      client,
      url_path,
    }
  }

  pub async fn execute(&self, file_paths: Vec<&str>) -> BenchmarkResult<AddFileResponseBody, BenchmarkError> {
    let (durations, successes, errors) = self.client.add_files(self.url_path, file_paths).await;
    let mut min = Duration::from_secs(u64::MAX);
    let mut max = Duration::from_secs(0);
    let mut avg = Duration::from_secs(0);

    for duration in durations {
      if duration < min {
        min = duration;
      }
      if duration > max {
        max = duration;
      }
      avg += duration;
    }

    let result = BenchmarkResult {
      _max_time: max,
      _min_time: min,
      _avg_time: avg,
      _successes: successes,
      _errors: errors,
    };

    result
  }
}