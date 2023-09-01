# splitflap-rs

Code rewrite for ScottBez1's [splitflap display project](https://github.com/scottbez1/splitflap/) in Rust. For fun and learning.

## Hardware Target

An ESP32 'TTGO' (with T-Display) is the target hardware for this project as this is the default board for the parent project.

https://www.lilygo.cc/products/lilygo%C2%AE-ttgo-t-display-1-14-inch-lcd-esp32-control-board
https://github.com/Xinyuan-LilyGO/TTGO-T-Display

ESP32-DOWDQ6 V3

## Development

It is easiest to do development from within a container for [esp32 rust](https://hub.docker.com/r/espressif/idf-rust/tags). For convenience, a [VSCode devcontainer](https://code.visualstudio.com/docs/devcontainers/containers) using this image is provided.

cargo build

espflash flash --port /dev/ttyACM0 --monitor target/xtensa-esp32-espidf/debug/splitflap-rs



https://lilymara.xyz/posts/images-esp32/
