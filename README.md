
# Project madness

A 2D lovecraftian investigation top-down game built on bevy engine, a rust ECS game engine.

## Prerequisites

Update your graphic drivers (works on nvidia-driver-580).
If using Ubuntu, go to Additional Drivers -> Using NVIDIA driver metapackage from nvidia-driver-580 (proprietary)
-> Apply changes -> Reboot

Install git :
```bash
sudo apt update
sudo apt upgrade
sudo apt install git
```

Clone this repository :
```bash
git clone git@github.com:Montblanc159/project-madness.git
```

Navigate to cloned repository :
```bash
cd project-madness
```

Install dependencies and init project :
```bash
sh scripts/init-project.sh
```

## Softwares

### Pureref
Visual references aggregator
[Download free version](https://www.pureref.com/download.php)

### Aseprite
Sprite creation tool
Prerequesite : [install docker](https://docs.docker.com/desktop/setup/install/linux/ubuntu/)

```bash
git clone git@github.com:Montblanc159/docker-build-aseprite.git
```

And follow README.md instructions

### LDTK
Tilemap builder
[Download here](https://ldtk.io/download/)

### Rust beads
Git synced local issue tracker
[Setup found here](https://github.com/Dicklesworthstone/beads_rust)

## Game usage

Game is built in bevy, a game engine developped in rust. It uses ECS (Entity Component System).

To play :
```bash
cargo run
```

To build binaries :
```bash
cargo build --release
```

## Code documentation

```bash
cargo doc
```
