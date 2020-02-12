# platformer

![GitHub](https://img.shields.io/github/license/nick96/platformer)
![CI](https://github.com/nick96/platformer/workflows/ci/badge.svg)

## How to run

To run the game, use

```
cargo run --features "vulkan"
```

on Windows and Linux, and

```
cargo run --features "metal"
```

on macOS.

For building without any graphics backend, you can use

```
cargo run --features "empty"
```

but be aware that as soon as you need any rendering you won't be able to run your game when using
the `empty` feature.
