# Rust and Ruin

A 2D Mech Hero RTS game built with Rust and Bevy, emphasizing real projectile physics and strategic action.

## 🎮 Game Overview

Rust and Ruin is a real-time strategy game where you control hero mechs in tactical combat. The game features realistic projectile physics, strategic positioning, and intense mech-vs-mech battles.

### Core Features
- **Hero Mech Control**: Direct control of powerful hero units with unique abilities
- **Physics-Based Combat**: Realistic projectile trajectories affected by gravity and momentum
- **Strategic Movement**: Click-to-move with intelligent pathfinding and tactical positioning
- **Real-Time Action**: Fast-paced combat requiring quick thinking and precise execution

## 🛠️ Technical Stack

- **Game Engine**: [Bevy](https://bevyengine.org/) - A data-driven game engine built in Rust
- **Language**: Rust - For performance, safety, and reliability
- **Physics**: bevy_rapier2d - For realistic 2D physics simulation
- **Architecture**: Entity Component System (ECS) pattern

## 🚀 Getting Started

### Prerequisites
- Rust (latest stable version)
- Cargo (comes with Rust)

### Installation
```bash
# Clone the repository
git clone https://github.com/yourusername/rust_and_ruin.git
cd rust_and_ruin

# Build the project
cargo build

# Run the game
cargo run
```

### Development Commands
```bash
# Run in release mode (optimized)
cargo run --release

# Run tests
cargo test

# Check code without building
cargo check

# Format code
cargo fmt

# Run linter
cargo clippy
```

## 🎯 Development Roadmap

### MVP - Minimal Playable Goal
A single Hero Mech smoothly moves to a clicked position, able to circle around a stationary Enemy Target. Hero Mech fires realistic physics-based projectiles toward the enemy on right-click.

### Development Steps

#### ✅ Step 1: Basic Bevy Setup
- Initialize Bevy App
- Set up 2D orthographic camera
- Create game window

#### ✅ Step 2: Spawn Hero Mech & Enemy Target
- Add sprite entities for Hero and Enemy
- Implement component system for entity differentiation
- Basic visual representation

#### ✅ Step 3: Click-to-Move System
- Left-click detection and world coordinate conversion
- Basic movement system (velocity-based)
- Circling behavior around enemies

#### ✅ Step 4: Projectile System
- Integrate bevy_rapier2d physics
- Spawn projectiles on right-click
- Physics-based projectile trajectories

#### ✅ Step 5: Collision Detection
- Implement collision system
- Projectile-enemy collision handling
- Visual feedback on impact

## 🏗️ Project Architecture

### Entity Component System (ECS)
The game uses Bevy's ECS architecture:

- **Entities**: Game objects (mechs, projectiles, enemies)
- **Components**: Data attached to entities (Position, Velocity, Health)
- **Systems**: Game logic that processes entities with specific components
- **Resources**: Global game state (settings, scores, game mode)

### Key Components
- `Hero`: Marks player-controlled mech entities
- `Enemy`: Marks enemy target entities
- `Projectile`: Projectile entities with physics properties
- `Velocity`: Movement data
- `Collider`: Collision detection boundaries

### Core Systems
- `MovementSystem`: Handles mech movement and pathfinding
- `InputSystem`: Processes mouse and keyboard input
- `ProjectileSystem`: Spawns and manages projectiles
- `PhysicsSystem`: Manages physics simulation
- `CollisionSystem`: Handles collision detection and response

## 📁 Project Structure
```
rust_and_ruin/
├── src/
│   ├── main.rs              # Entry point and app setup
│   ├── components.rs        # Game component definitions
│   ├── resources.rs         # Global game resources
│   └── systems/            # Game systems
│       ├── movement.rs
│       ├── input.rs
│       ├── projectile.rs
│       └── collision.rs
├── assets/                  # Game assets (sprites, sounds)
├── docs/                    # Additional documentation
├── tasks/
│   └── todo.md             # Development task tracking
├── Cargo.toml              # Rust dependencies
└── README.md               # This file
```

## 🤝 Contributing

This project is in early development. Contributions are welcome! Please follow these guidelines:

1. Follow Rust coding conventions
2. Use Bevy's ECS patterns
3. Keep systems small and focused
4. Add tests for new functionality
5. Update documentation as needed

## 📝 License

[License information to be added]

## 🙏 Acknowledgments

- Bevy Engine community
- Rust gamedev community
