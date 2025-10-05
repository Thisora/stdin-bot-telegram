use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Set log level to error only
    #[arg(short, default_value_t = false)]
    pub quiet: bool,

}