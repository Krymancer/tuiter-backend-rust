#[derive(clap::Parser)]
pub struct Config {
  #[clap(short, long)]
  pub database_url: String,
}