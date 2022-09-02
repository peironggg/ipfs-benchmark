mod client;
mod response;

const BASE_URL: &str = "http://127.0.0.1:5001/api/v0";

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage : benchmark <file_path_to_add>");
        std::process::exit(1);
    }
    let file_path: &str = &args[1];

    let benchmark_client = client::BenchmarkClient {
        http_client: reqwest::Client::new(),
    };
    let (successes, errors) = benchmark_client.add_files(BASE_URL, vec![file_path]).await;

    println!("successes: {:?}", successes);
    println!("errors: {:?}", errors);
}
