# mesh_shader

This example renders a triangle to a window with mesh shaders, while showcasing most mesh shader related features(task shaders, payloads, per primitive data).

Mesh shaders require `Features::EXPERIMENTAL_MESH_SHADER`. On adapters that do
not expose it, the example uses an equivalent vertex shader pipeline so it can
still run. `wgpu` does not perform this translation automatically; applications
need to provide their own fallback path.

## To Run

```
cargo run --bin wgpu-examples mesh_shader
```
