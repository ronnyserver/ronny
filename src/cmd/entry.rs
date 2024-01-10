use crate::io::serve;
use clap::Parser;
/// Simple program to start a http server
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(long, default_value = "0.0.0.0")]
    host: String,

    /// Number of times to greet
    #[arg(long, default_value_t = 8000)]
    port: u16,
}

pub async fn entry() {
    let args = Args::parse();

    println!("Listening on {}:{}", args.host, args.port);
    serve(args.host, args.port).await;
}
