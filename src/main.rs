// Main crate, only defines the CLI and calls stuff in the Lib crate based on CLI input.
//
// Detailed treatment of how to structure a project:
// https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html

use std::time::Instant;
// Clap is used to define the cli declaratively
use clap::{Parser, Subcommand, ValueEnum};
use srt::output::image::Image;
use srt::render::helloworld::HelloWorld;
use srt::render::pathtracer::PathTracer;
use srt::render::renderer::Renderer;
use srt::scenes;

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

// Available rendering algorithms
#[derive(ValueEnum, Clone, Default, Debug)]
pub enum Algorithm {
    #[default]
    HelloWorld,
    PathTracer,
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
            // TODO scene from cli argument/flag
            // let scene = scenes::single_orb::generate();
            let scene: srt::scene::scene::Scene = scenes::small_orb_with_ground::generate();
            // let scene: srt::scene::scene::Scene = scenes::small_orb_in_front_of_larger_one::generate();

            let mut renderer = get_renderer(&algorithm);

            let mut output = Image::new(*width, *height, output_image.to_string());

            renderer.render(&scene, *width, *height, &mut output);
        }
        _ => {
            println!("Specify a subcommand");
        }
    }

    println!("Command completed in {:.2?}", start.elapsed());
}

// Returns a renderer object for the specified Algorithm
fn get_renderer(algorithm: &Algorithm) -> Box<dyn Renderer> {
    match &algorithm {
        Algorithm::HelloWorld => Box::new(HelloWorld::new()),
        Algorithm::PathTracer => Box::new(PathTracer::new()),
    }
}
