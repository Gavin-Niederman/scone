use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "This game was written with the Scone game engine"
)]
pub struct Args {
    /// The number of ticks to run per second
    #[arg(short, long, default_value_t = 66.0)]
    pub tps: f32,

    /// The log level for the logger to use
    #[arg(short, long, default_value_t = log::LevelFilter::Warn)]
    pub log_level: log::LevelFilter,
}
