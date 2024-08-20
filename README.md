# Game Clip

Game Clip is a Rust-based utility that continuously records your screen, storing the last few seconds (or minutes) of footage as configured. When you press a designated key combination, it converts the recent footage into a GIF using [gifski](https://github.com/ImageOptim/gifski).

## Features
- **Continuous Recording:** Always captures the last N seconds of your screen.
- **Instant GIF Creation:** Save gameplay highlights as GIFs with a simple key press.
- **Configurable Hotkeys:** Easily set your preferred key combinations.

## Usage
- Press <kbd>7</kbd> <kbd>8</kbd> <kbd>9</kbd> to capture a GIF.
- Press <kbd>Num 7</kbd> <kbd>Num 8</kbd> <kbd>Num 9</kbd> to capture raw footage.

## Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/lilBunnyRabbit/game-clip
   ```
2. Build the project:
   ```bash
   cargo build --release
   ```
3. Run the executable:
   ```bash
   ./target/release/game-clip
   ```

## Configuration

The `config.yaml` file allows you to customize the behavior of the `game-clip` application. The following settings are available:

- **quality**: Sets the GIF quality (0-100).
- **fast**: Toggles gifski's fast mode (default: true).
- **repeat**: Controls GIF looping behavior (e.g., infinite).
- **fps**: Frames per second (default: 60).
- **duration**: Duration of the recording in seconds (default: 3).
- **width/height**: GIF dimensions in pixels (default: 640x360).
- **path**: Output path for saved GIFs (default: `./tmp/`).
- **display**: Selects the display to record (0 for primary, 1 for secondary, etc.).

If the file is missing, it will be created with default values.

## Contributing
Contributions are welcome! Feel free to submit issues or pull requests.
