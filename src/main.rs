// Detailed treatment of how to structure a project:
// https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html

// `mod` is NOT an include or import but a declaration stating that a module exists.
// The definition can be inline using {} OR in a subdirectory containing a mod.rs file.
mod output;
mod render;
mod scene;

use render::Algorithm;
use render::Renderer;
use render::helloworld::HelloWorld;
use render::pathtracer::PathTracer;
use std::time::Instant;

// Clap is used to define the cli declaratively
use crate::scene::scene::Scene;
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
        output_image: String,
        #[arg(short, long, default_value_t = Algorithm::PathTracer, value_enum)]
        algorithm: Algorithm,
        #[arg(long, default_value_t = DEFAULT_WIDTH)]
        width: u32,
        #[arg(long, default_value_t = DEFAULT_HEIGHT)]
        height: u32,
    },
}

fn main() {
    let cli = Cli::parse();

    let start = Instant::now();

    match &cli.command {
        Some(Commands::Render {
            algorithm,
            output_image,
            width,
            height,
        }) => {
            let scene = Scene {
                name: "Empty".to_string(),
            };

            let mut renderer = get_renderer(&algorithm);

            let mut output = output::image::Image::new(*width, *height, output_image.to_string());

            renderer.render(&scene, *width, *height, &mut output);
        }
        _ => {
            println!("Specify a subcommand");
        }
    }

    println!("Command completed in {:.2?}", start.elapsed());
}

fn get_renderer(algorithm: &Algorithm) -> Box<dyn Renderer> {
    match &algorithm {
        Algorithm::HelloWorld => Box::new(HelloWorld::new()),
        Algorithm::PathTracer => Box::new(PathTracer::new()),
    }
}
