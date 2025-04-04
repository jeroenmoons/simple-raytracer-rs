mod render;

use crate::render::pathtracer::PathTracer;
use clap::{Parser, Subcommand};

// Default output image dimensions
const DEFAULT_WIDTH: u32 = 800;
const DEFAULT_HEIGHT: u32 = 600;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

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
