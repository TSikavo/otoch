use std::process::{Command};
use clap::{Parser, Subcommand};

use otoch::ServiceCommands;

#[derive(Parser)]
#[command(name = "otoch", version, author = "tsikavo")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "app-id", about = "Show the application id")]
    AppId {},
    #[command(name = "service", about = "Main background service")]
    Service{
        #[command(subcommand)]
        subcommand: ServiceCommands,
    },
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
        Commands::AppId {} => {
            println!(env!("APP_ID"));
        },
        Commands::Service { subcommand } => {
            let args = match subcommand {
                ServiceCommands::Start { args } => (subcommand.as_str(), Some(args.clone())),
                ServiceCommands::Stop => (subcommand.as_str(), None),
                ServiceCommands::Restart { args } => (subcommand.as_str(), Some(args.clone())),
                ServiceCommands::Status { minimal } => {
                    let args = if *minimal {
                        Some(vec!["-m".to_string()])
                    } else { None };
                    (subcommand.as_str(), args)
                },
                _ => ("", None)
            };

            let mut command = Command::new("otoch-service");
            command.arg(args.0);
            if let Some(args) = args.1 {
                command.args(args);
            }

            let output = command.output().expect("failed to execute process (otoch-service)");

            if !output.status.success() {

            }
            print!("{}", String::from_utf8_lossy(&output.stdout));
            eprint!("{}", String::from_utf8_lossy(&output.stderr));
        }
        Commands::Manager {} => {
            todo!()
        },
        Commands::Open { window_name, target_monitor, position } => {
            dbg!(window_name, target_monitor, position);
            todo!()
        }
    }
}