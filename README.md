# Drone Agility Challenge


https://github.com/Samuel-Bowden/drone-agility-challenge/assets/91887909/5e9524ad-5642-48f1-ab1f-f91590e8c10b

The aim of the game is to fly the drone from the red to the green podium without colliding into anything else.

## Key-Mapping

The movement of the drone is mapped to WASD. W/S activates the bottom and top thrusters. A/D rotates the drone anti-clockwise and clockwise.

## Playing

The game can be played on my website at https://samuelbowden.com/games/drone-agility-challenge, or it can be installed natively to devices running Windows, Linux and macOS with the instructions below.

## Installation

See the releases section for pre-compiled versions of the game. If a precompiled version is not available for your platform, please follow the compilation instructions below.

## Compilation

Firstly, a rustup installation is required. Instructions for installing this can be found at https://www.rust-lang.org/tools/install.

Depending on your platform, some other dependencies may need to be installed, which are detailed in this guide:
https://bevyengine.org/learn/book/getting-started/setup/

Once the above steps are completed, clone this repository and build+run using cargo:
```bash
git clone https://github.com/Samuel-Bowden/drone-agility-challenge
cd drone-agility-challenge
cargo run --release
```
