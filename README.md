# Raytracer

This project is a simple Rust translation of the [Ray Tracing](https://raytracing.github.io/) book series. It renders 3D images by tracing the path of light rays as they interact with a set of abstract objects.

It is an incomplete representation of the features outlined in the series, and should be considered a WIP.

## How to Run

```
$ cargo build
$ cargo run -- -o output.png
```
The resulting image will be rendered to `output.png`.

## Supported Features

- [x] .PPM File Support
- [x] .PNG File Support
- [x] Antialiasing
- [x] Diffuse Materials
- [x] Glass
- [x] Metal
- [x] Depth of Field
- [ ] Configurable Scenes
- [ ] Parallel Rendering
- [ ] Motion Blur
- [ ] Textures
- [ ] Lighting
