# Ray Tracer in Rust

A path tracer built in Rust following [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html) by Peter Shirley, with BVH acceleration added on top from [_Ray Tracing: The Next Week_](https://raytracing.github.io/books/RayTracingTheNextWeek.html).
Also includes parallelism with a custom ThreadPool made by myself

## Features

- Lambertian, metal, and dielectric materials
- Positionable camera with depth-of-field
- BVH (Bounding Volume Hierarchy) acceleration structure with random-axis sorting
- Custom thread pool with atomic tile-based work stealing
- Min-heap reordering buffer to emit scanlines in order from out-of-order tile results

## Architecture

```
src/
├── main.rs          # Scene setup and entry point
├── camera.rs        # Render orchestration, tile dispatch, PPM output
├── thread_pool.rs   # Worker pool — atomic counter, per-worker tile loop
├── tiles.rs         # Tile / ResultTile definitions
├── bvh.rs           # BVH node — recursive build + hit traversal
├── hittable.rs      # Hittable trait + HittableList
├── sphere.rs        # Sphere geometry + AABB construction
├── aabb.rs          # Axis-aligned bounding box + slab hit test
├── interval.rs      # Interval utility (min/max, surrounds, contains)
├── material.rs      # Lambertian, Metal, Dielectric scatter
├── ray.rs           # Ray (origin + direction + at(t))
└── vector.rs        # Vec3 / Color
```

## How the parallelism works

The image is split into horizontal tiles. A `ThreadPool` spawns one worker per logical core. Workers pull the next tile index from a shared `Arc<AtomicUsize>` counter — no mutex, no channel for work distribution, just a fetch-add. Each worker renders its tile and sends the result back via `mpsc::channel`.

Because tiles finish out of order, the main thread uses a **min-heap** keyed on tile ID to buffer results and emit them in scanline order, keeping PPM output correct regardless of scheduling.

```
AtomicUsize counter ──► Worker 0 ──┐
                    ──► Worker 1 ──┤ mpsc::Sender<ResultTile>
                    ──► Worker N ──┘
                                    │
                            mpsc::Receiver
                                    │
                              Min-heap (id)
                                    │
                            stdout (PPM)
```

## How the BVH works

Objects are recursively split into two groups by sorting along a randomly chosen axis and splitting at the midpoint. Each node stores an AABB that tightly wraps its subtree. Ray traversal tests the AABB first — a miss prunes the entire subtree in one cheap slab test, reducing ray-scene intersection from O(n) to O(log n).

## Benchmarks

Three scenes of increasing complexity. Large scene: ~1600 spheres, 1200px wide, 250 samples per pixel.

### Raw times

| Stage | Small | Medium | Large |
|---|---|---|---|
| Baseline | 333.68 / 317.04 / 335.31 ms | 73.60 / 75.19 / 74.24 s | 2196.72 s |
| Parallel only | 43.73 / 48.79 / 49.14 ms | 9.51 / 9.90 / 11.65 s | 345.32 / 329.43 s |
| Parallel + BVH | 65.94 / 60.38 / 60.08 ms | 2.92 / 2.97 / 3.10 s | 30.46 / 35.69 / 36.74 s |

### Speedup vs baseline (averaged)

| Stage | Small | Medium | Large |
|---|---|---|---|
| Parallel only | ~6.8× | ~7.5× | ~6.4× |
| Parallel + BVH | ~5.5× | ~25.1× | ~64.2× |

## Building

```bash
cargo build --release
cargo run --release
```

## Citations

[_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
[_Ray Tracing: The Next Week_](https://raytracing.github.io/books/RayTracingTheNextWeek.html)
