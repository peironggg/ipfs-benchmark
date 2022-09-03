use crate::client::{BenchmarkClient, BenchmarkError};
use crate::response::body::AddFileResponseBody;
use std::time::{Duration, Instant};

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
    let now = Instant::now();
    let (successes, errors) = self.client.add_files(self.url_path, file_paths).await;
    let elapsed_time = now.elapsed();

    let result = BenchmarkResult {
      _max_time: elapsed_time,
      _min_time: elapsed_time,
      _avg_time: elapsed_time,
      _successes: successes,
      _errors: errors,
    };

    result
  }
}