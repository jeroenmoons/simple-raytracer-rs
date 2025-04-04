# simple-raytracer-rs
A basic ray tracer - or path tracer, more accurately - written in Rust.

A *ray tracer* shoots a single ray per pixel from the camera into the scene and follows reflection/refraction 
deterministically. This approach is viable for real time rendering. 

A *path tracer* uses a stochastic approach, randomly sampling multiple paths per pixel over many 
iterations to approximate global illumination. A path tracer can result in photorealistic results given correct 
modelling and enough iterations. This approach is a lot more computationally intensive and is more suitable for offline
rendering.

This project attempts a basic implementation - with very restricted geometry, materials, and techniques - serving mainly 
as a learning opportunity for the Rust language.

## Usage

```bash
# Run the renderer using defaults to produce an PNG image "out.png"
cargo run render out.png

# Run a different algorithm, the hello-world renderer in this case
cargo run render helloworld.png -a hello-world

# Produce a 400x300 png image
cargo run render out_400x300.png --width 400 --height 300
```