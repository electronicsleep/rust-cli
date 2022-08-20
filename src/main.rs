use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "rust-cli")]
#[clap(about = "rust-cli", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,

    /// Env
    #[clap(short, long, default_value = "qa")]
    env: String,

    /// Region
    #[clap(short, long, default_value = "us-west-1")]
    region: String,

    /// Verbose
    #[clap(short, long)]
    verbose: bool,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Check Echo
    #[clap(arg_required_else_help = true)]
    CheckEcho {
        /// Service
        service: String,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::CheckEcho { service } => {
            println!("check-echo env: {} region: {} verbose: {} service: {}", args.env, args.region, args.verbose, service);
            check_echo()
        }
    }
}

fn check_echo() {
    println!("check_echo")
}
