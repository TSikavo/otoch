use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "otoch", version = "0.1.0", author = "tsikavo")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "manager", about = "Open otoch manager")]
    Manager {},
    #[command(name = "open", about = "Open a new window")]
    Open {
        #[arg(short = 'n', long = "name", help = "Name of the window you want to open", required = true)]
        window_name: String,
        #[arg(short = 'm', long = "monitor", help = "ID of the monitor on which the window will open", default_value_t = 0)]
        target_monitor: u8,
        #[arg(short = 'p', long = "position", help = "Position of the window (WIDTHxHEIGHT)", default_value = "0x0", value_parser = parse_position)]
        position: [u16; 2],
    }
}

fn parse_position(s: &str) -> Result<[u16; 2], String> {
    let parts: Vec<&str> = s.split("x").collect();
    if parts.len() != 2 {
        return Err(format!("Invalid format: {s}. Use 'WIDTHxHEIGHT'"));
    }

    let width = parts[0].parse().map_err(|_| format!("'{}' is not an integer", parts[0]))?;
    let height = parts[1].parse().map_err(|_| format!("'{}' is not an integer", parts[1]))?;

    Ok([width, height])
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Manager {} => {

        },
        Commands::Open { window_name, target_monitor, position } => {

        }
    }
}
