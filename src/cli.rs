use clap::Parser;

#[derive(Parser)]
#[command(name = "cache-proxy")]
#[command(version = "0.1.0")]
#[command(about = "A simple cache proxy server")]
pub struct Args {
    #[clap(short, long, default_value = "3000")]
    pub port: u16,

    #[clap(short, long, default_value = "http://localhost")]
    pub origin: String,

    #[clap(long("clear-cache"))]
    pub clear_cache: bool,
}
