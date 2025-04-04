mod render;

// `crate::` is the starting point for paths pointing to modules in the current crate
use crate::render::pathtracer::PathTracer;
// Clap is used to define the cli declaratively
use clap::{Parser, Subcommand};

// Default output image dimensions
const DEFAULT_WIDTH: u32 = 800;
const DEFAULT_HEIGHT: u32 = 600;

// Clap maps cli command string to this struct
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

// Clap feature to map subcommand args to an enum variant
#[derive(Subcommand)]
enum Commands {
    Render {
        // TODO:
        //  - input scene
        //  - output image path
        #[arg(long, default_value_t = DEFAULT_WIDTH)]
        width: u32,
        #[arg(long, default_value_t = DEFAULT_HEIGHT)]
        height: u32,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Render { width, height }) => {
            let renderer = PathTracer::new();
            renderer.render(width, height);
        }
        _ => {
            println!("Specify a subcommand");
        }
    }
}
