# gitlogue

A beautiful Git commit history screensaver that brings your past development sessions back to life.

Watch your code being typed out again with syntax highlighting, just like you wrote it the first time.

## Features

- 🎬 **Animated Commit Replay** - Watch commits being recreated with realistic typing animations
- 🎨 **Syntax Highlighting** - Support for 26 programming languages via tree-sitter
- 🌳 **File Tree View** - Directory structure with change statistics
- 📊 **Commit Metadata** - Author, date, message, and change details
- 🎯 **Multiple Modes** - Random commit playback or view a specific commit
- ⚡ **Fast & Lightweight** - Built with Rust for performance

## Supported Languages

Rust, TypeScript, JavaScript, Python, Go, Ruby, Swift, Kotlin, Java, PHP, C#, C, C++, Haskell, Dart, Scala, Clojure, Zig, Elixir, Erlang, HTML, CSS, JSON, Markdown, YAML, XML

## Installation

### From Source

```bash
git clone https://github.com/unhappychoice/gitlogue.git
cd gitlogue
cargo install --path .
```

### Using Cargo

```bash
cargo install gitlogue
```

## Usage

### Screensaver Mode (Random Commits)

Navigate to a Git repository and run:

```bash
gitlogue
```

This will randomly select commits and replay them with animations.

### View a Specific Commit

```bash
gitlogue --commit <commit-hash>
```

### Options

- `--commit <hash>` - Display a specific commit instead of random playback
- `--help` - Show help information

### Controls

- `q` or `Ctrl+C` - Quit the application

## How It Works

gitlogue reads your Git repository history and recreates the development experience:

1. Selects commits from your repository (randomly or specified)
2. Analyzes file changes and diffs
3. Uses tree-sitter for syntax highlighting
4. Animates the typing of code changes line by line
5. Displays file trees, metadata, and commit information

## Development

### Requirements

- Rust 1.70 or later
- Git

### Building

```bash
cargo build --release
```

### Running Tests

```bash
cargo test
```

### Running Examples

```bash
cargo run --example test_highlighter
```

## Roadmap

See the [v0.1.0 Milestone](https://github.com/unhappychoice/gitlogue/milestone/1) for planned features.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

ISC License - see [LICENSE](LICENSE) file for details.

## Author

[@unhappychoice](https://github.com/unhappychoice)
