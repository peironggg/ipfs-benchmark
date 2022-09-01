use reqwest::{multipart, Client, Body};
use crate::response::body::{AddFileResponseBody};
use tokio::{fs::File};
use tokio_util::codec::{BytesCodec, FramedRead};

// Type Aliases
type Result<T> = std::result::Result<T, BenchmarkError>;

// Errors
#[derive(Debug)]
pub enum BenchmarkError {
  IOError(std::io::Error),
  ReqwestError(reqwest::Error),
}

impl From<std::io::Error> for BenchmarkError {
  fn from(error: std::io::Error) -> Self {
      BenchmarkError::IOError(error)
  }
}

impl From<reqwest::Error> for BenchmarkError {
  fn from(error: reqwest::Error) -> Self {
      BenchmarkError::ReqwestError(error)
  }
}


pub struct BenchmarkClient {
  pub http_client: Client,
}

impl BenchmarkClient {
  pub async fn add_file(self, url_path: &str, file_path: &str) -> Result<AddFileResponseBody> {
    let file = File::open(file_path).await?;
    let stream = FramedRead::new(file, BytesCodec::new());
    let body = Body::wrap_stream(stream);
  
    let part = multipart::Part::stream(body);
    let form = multipart::Form::new().part("file", part);
    let full_url = format!("{}/add", url_path);
    let response = self.http_client.post(full_url).multipart(form).send().await?;
    
    Ok(response.json().await?)
  }
}

trait RemoveFile {
  fn remove_file(self, cid: &str) -> Result<()>;
  fn remove_files(self, cids: &[&str]) -> Result<()>;
}