# Milo Yip's light2d in Rust

This project is an implementation of [Milo Yip's light2d](https://github.com/miloyip/light2d) in Rust.

## Usage

All cases are implemented in one project. Use command to run different cases.

```shell
# Run with Cargo,
cargo run --release -- [--output <output image>] <command> [<args>]
# or use the binary directly.
light2d-rust [--output <output image>] <command> [<args>]
```

The `<command>` can be one of the following case names.

* **basic** The basic example.
* **csg** The constructive solid geometry example.
* **shapes** Example of many shapes.
* **scene** A simple scene implementation.
* **reflection** A simple reflection example.

## Gallery

<p>
  <img src="https://github.com/chengluyu/light2d-rust/blob/master/output/basic.png?raw=true" width="20%" />
  <img src="https://github.com/chengluyu/light2d-rust/blob/master/output/csg.png?raw=true" width="20%" />
  <img src="https://github.com/chengluyu/light2d-rust/blob/master/output/shapes.png?raw=true" width="20%" />
  <img src="https://github.com/chengluyu/light2d-rust/blob/master/output/scene.png?raw=true" width="20%" />
  <img src="https://github.com/chengluyu/light2d-rust/blob/master/output/reflection.png?raw=true" width="20%" />
</p>
