# Rustris

**Rustris** is a Tetris clone built using the Rust programming language and the Bevy game engine. Enjoy the classic block-stacking game with smooth animations, crisp controls, and a modern twist.

## How to Play

The goal of **Rustris** is to clear lines by strategically stacking different-shaped blocks as they fall from the top of the screen. Once a line is fully filled with blocks, it disappears, and you earn points. The game speeds up over time, challenging you to keep up and prevent the blocks from reaching the top of the screen.

### Controls

- **Move Left**: `Left Arrow Key`
- **Move Right**: `Right Arrow Key`
- **Rotate Block**: `Up Arrow Key`
- **Soft Drop**: `Down Arrow Key` (move the block down faster)
- **Hard Drop**: `Space` (instantly drops the block to the bottom)
- **Hold Block**: `C` (hold the current block for later use)

### Gameplay Features

- **Block Holding**: Press `C` to hold the current block and use it later. You can only hold one block at a time. If a block is already held, pressing `C` will swap the held block with the current one.
- **Block Rotation**: Press the `Up Arrow Key` to rotate the current block clockwise. The game supports wall kicks, allowing blocks to rotate even when adjacent to the edges of the playfield.
- **Movement**: Use the `Left` and `Right Arrow Keys` to move the current block horizontally.
- **Hard Drop**: Press `Space` to instantly drop the block to the bottom, locking it in place.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/) (latest stable version)

### Running the Game

```bash
git clone https://github.com/yourusername/rustris.git
cd rustris
cargo run -r
```

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue for any bugs or suggestions.

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Acknowledgements

Developed with Rust and Bevy.
Inspired by the original Tetris game, created by Alexey Pajitnov in 1984.
