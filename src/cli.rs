use crate::task::TaskStatus;
use clap::Args;
use clap::Parser;
use clap::Subcommand;

use crate::utils::*;

#[derive(Parser, Debug)]
#[command(
    name = "todoing",
    version = "0.1",
    about = "Simple task manager built in rust"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Add { description: String },
    Remove { id: u32 },
    Update { id: u32, description: String },
    MarkTodo { id: u32 },
    MarkDoing { id: u32 },
    MarkDone { id: u32 },
    List,
    Clean,
}

pub fn handle_commands() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Add { description } => add_task(description.clone()),
        Commands::Remove { id } => remove_task(id),
        Commands::Update { id, description } => update_task(id, description),
        Commands::List => list_tasks(),
        Commands::MarkTodo { id } => mark_tasks(id, TaskStatus::Todo),
        Commands::MarkDoing { id } => mark_tasks(id, TaskStatus::Doing),
        Commands::MarkDone { id } => mark_tasks(id, TaskStatus::Done),
        Commands::Clean => clean_tasks(),
    }
}
