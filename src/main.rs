mod tui;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "rust-cli")]
#[clap(author, version, about, long_about = None)]
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
    /// Echo
    #[clap(arg_required_else_help = true)]
    Echo {
        /// Service
        service: String,
    },
    /// Tui
    #[clap(arg_required_else_help = true)]
    Tui {
        /// Mode
        mode: String,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Echo { service } => {
            println!(
                "echo: env: {} region: {} verbose: {} service: {}",
                args.env, args.region, args.verbose, service
            );
            echo(service);
        }
        Commands::Tui { mode } => {
            println!("tui: mode: {} verbose: {}", mode, args.verbose);
            let result = tui::tui();
            println!("result {:?}", result)
        }
    }
}

fn echo(service: String) {
    println!("echo {}", service);
}
