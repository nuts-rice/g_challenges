use clap::Parser;
use hyper::Request;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'i', long)]
    inputOption: Option<String>,
    #[clap(short = 'o', long)]
    outputOption: Option<String>,
}

fn build_request(args: Args) -> Result<()> {
    let mut builder = Request::builder();
    builder.method("POST");
    builder.uri("http://localhost:8080");
    builder.header("content-type", "application/json");
    builder.body(Body::from(args.inputOption.unwrap())).unwrap()
}
#[tokio::main]
async fn main() -> Result<()> { 
    // tracing_subscriber::fmt().with_env_filter("e2e=warning").init();
unimplemented!()
}
