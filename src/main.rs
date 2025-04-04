// `mod` is NOT an include or import but a declaration stating that a module exists.
// The definition can be inline using {} OR in a subdirectory containing a mod.rs file.
mod render;

// `crate::` is the starting point for paths pointing to modules in the current crate.
// Detailed treatment of how to structure a project:
// https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
use render::pathtracer::PathTracer;

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
