// Main crate, only defines the CLI and calls stuff in the Lib crate based on CLI input.
//
// Detailed treatment of how to structure a project:
// https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html

use std::time::Instant;
// Clap is used to define the cli declaratively
use clap::{Parser, Subcommand, ValueEnum};
use srt::output::output::OutputType;
use srt::render::helloworld::HelloWorld;
use srt::render::pathtracer::PathTracer;
use srt::render::renderer::Renderer;
use srt::scene::scene::Scene;
use srt::scenes;

// Default output image width. Height is determined by this and the camera aspect ratio
const DEFAULT_WIDTH: u32 = 800;

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
        scene: SceneName,
        #[arg(short, long, default_value = "out/out.png")]
        output_image: String,
        #[arg(short, long, default_value_t = Algorithm::PathTracer, value_enum)]
        algorithm: Algorithm,
        #[arg(long, default_value_t = DEFAULT_WIDTH)]
        width: u32,
    },
}

// Available rendering algorithms
#[derive(ValueEnum, Clone, Default, Debug)]
pub enum Algorithm {
    #[default]
    HelloWorld,
    PathTracer,
}

// Available scenes
#[derive(ValueEnum, Clone, Copy, Debug)]
enum SceneName {
    Empty,
    SingleOrb,
    SmallOrbInFrontOfLargerOne,
    OrbWithGround,
}

fn main() {
    let cli = Cli::parse();

    let start = Instant::now();

    match &cli.command {
        Some(Commands::Render {
            algorithm,
            scene,
            output_image,
            width,
        }) => {
            let scene = select_scene(*scene);

            let mut renderer = select_renderer(&algorithm);

            match renderer.render(&scene, String::from("main"), *width, OutputType::PNG) {
                Ok(output) => output.save(output_image),
                Err(err) => panic!("Error rendering {err}"),
            }
        }
        _ => {
            println!("Specify a subcommand");
        }
    }

    println!("Command completed in {:.2?}", start.elapsed());
}

// Returns a renderer object for the specified Algorithm
fn select_renderer(algorithm: &Algorithm) -> Box<dyn Renderer> {
    match &algorithm {
        Algorithm::HelloWorld => Box::new(HelloWorld::new()),
        Algorithm::PathTracer => Box::new(PathTracer::new(100, 50)),
    }
}

fn select_scene(name: SceneName) -> Scene {
    match name {
        SceneName::Empty => scenes::empty::generate(),
        SceneName::SingleOrb => scenes::single_orb::generate(),
        SceneName::SmallOrbInFrontOfLargerOne => {
            scenes::small_orb_in_front_of_larger_one::generate()
        }
        SceneName::OrbWithGround => scenes::orb_with_ground::generate(),
    }
}
