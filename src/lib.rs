pub mod manager;

use clap::{Subcommand, ArgGroup};

#[derive(Subcommand)]
pub enum ServiceCommands {
    #[command(name = "start", about = "Use to start the background service")]
    Start {
        #[arg(required = false, help = "Additional arguments")]
        args: Vec<String>,
    },
    #[command(name = "stop", about = "Use to stop the background service")]
    Stop,
    #[command(name = "restart", about = "Use to restart the background service")]
    Restart {
        #[arg(required = false, help = "Additional arguments")]
        args: Vec<String>,
    },
    #[command(name = "status", about = "Show the status of the background service")]
    Status {
        #[arg(short = 'm', long = "minimal", required = false, help = "Show minimal (only enabled or disabled)", default_value_t=false)]
        minimal: bool,
    },
    #[command(name = "unsafe", hide = true)]
    #[command(group(
        ArgGroup::new("action")
            .required(true)
            .args(&["start", "stop", "restart"]),
    ))]
    Unsafe {
        #[arg(long = "start", group = "action")]
        start: bool,
        #[arg(long = "stop", group = "action")]
        stop: bool,
        #[arg(long = "restart", group = "action")]
        restart: bool,
        #[arg(required = false, help = "Additional arguments")]
        args: Vec<String>,
    }
}

impl ServiceCommands {
    pub fn as_str(&self) -> &'static str {
        match self {
            ServiceCommands::Start { .. } => { "start" },
            ServiceCommands::Stop => { "stop" },
            ServiceCommands::Restart { .. } => { "restart" },
            ServiceCommands::Status { minimal: _ } => { "status" },
            ServiceCommands::Unsafe { start: _, stop: _, restart: _, args: _, } => { "unsafe" },
        }
    }
}