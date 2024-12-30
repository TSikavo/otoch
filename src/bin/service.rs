use std::env;
use std::process::Command;
use clap::{Parser};
use adw::{Application};
use adw::gio::ApplicationHoldGuard;
use adw::prelude::{ApplicationExt, ApplicationExtManual};

use otoch::ServiceCommands as Commands;

#[derive(Parser)]
#[command(name = "otoch-service", version, author = "tsikavo")]
struct  Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Start { args } => {
            run_unsafe("--start", Some(args))
        },
        Commands::Stop => {
            run_unsafe("--stop", None)
        },
        Commands::Restart { args } => {
            run_unsafe("--restart", Some(args))
        },
        Commands::Status { minimal } => {
            match minimal {
                true => { println!("{}", status().as_str()); },
                false => { println!("Status: {}", status().as_str()); }
            }
        },
        Commands::Unsafe { start: start_a, stop: stop_a, restart: restart_a, args } => {
            if *start_a {
                start(args)
            } else if *stop_a {
                stop()
            } else if *restart_a {
                restart(args)
            }
        }
    }

    fn run_unsafe(main_arg: &str, args: Option<&Vec<String>>) {
        let mut command = Command::new(env::current_exe().expect("Failed to get current executable path"));

        command.arg(Commands::Unsafe {start: false, stop: false, restart: false, args: Vec::new()}.as_str()).arg(main_arg);
        if let Some(args) = args {
            command.args(args);
        }

        command.spawn().expect("Failed to spawn child process");
    }

}

fn start(args: &Vec<String>) {
    let service = Service::new();
    service.start(args);
}

fn stop() {
    todo!()
}

fn restart(args: &Vec<String>) {
    dbg!(args);
    todo!()
}

pub fn status() -> Status  {
    todo!()
}

pub enum Status {
    Enabled,
    Disabled,
}

impl Status {
    pub fn as_str(&self) -> &'static str {
        match self {
            Status::Enabled => { "enabled" },
            Status::Disabled => { "disabled" },
        }
    }
}

struct Service {
    app: Application,
    _hold_guard: ApplicationHoldGuard
}

impl Service {
    fn new() -> Self {
        let app = Application::builder()
            .application_id(env!("APP_ID"))
            .build();
        let hold_guard = app.hold();

        app.connect_activate(move |_| {

        });

        Service {
            app,
            _hold_guard: hold_guard
        }
    }

    fn start(&self, args: &Vec<String>) {
        self.app.run_with_args(args);
    }
}