# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

rust_and_ruin is a top-down 2D Mech Hero RTS game built with Rust and the Bevy game engine, emphasizing real projectile physics and strategic action.

## Development Commands

Once the Rust project is initialized, common commands will include:

```bash
# Build the project
cargo build

# Run the game in development mode
cargo run

# Build and run in release mode (optimized)
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

## Project Architecture

As this is a Bevy-based game project, the expected architecture will likely follow Bevy's Entity Component System (ECS) pattern:

- **Systems**: Game logic organized into systems that process entities with specific components
- **Components**: Data structures attached to entities (e.g., Position, Velocity, Health)
- **Resources**: Global game state (e.g., game settings, score, current level)
- **Plugins**: Modular features organized as Bevy plugins
- **Assets**: Game assets (sprites, sounds, configs) typically in an `assets/` directory

## Key Considerations

When developing in this codebase:
- Bevy uses an ECS architecture - prefer composition over inheritance
- Systems should be small and focused on a single responsibility
- Use Bevy's built-in physics or integrate a physics engine for projectile mechanics
- Consider performance implications for RTS games (many units on screen)
- Organize game features as separate Bevy plugins for modularity

## Standard workflow

1. First think through the problem, read the codebase for relevant files, and write a plan to tasks/todo.md.
2. The plan should have a list of todo items that you can check off as you complete them
3. Before you begin working, check in with me and I will verify the plan.
4. Then, begin working on the todo items, marking them as complete as you go.
5. Please every step of the way just give me a high level explanation of what changes you made
6. Make every task and code change you do as simple as possible. We want to avoid making any massive or complex changes. Every change should impact as little code as possible. Everything is about simplicity.
7. Finally, add a review section to the todo.md file with a summary of the changes you made and any other relevant information.