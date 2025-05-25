# Ask - High-Performance Claude AI CLI

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue?style=for-the-badge)

A fast and efficient command-line interface for interacting with Claude AI, built with Rust for maximum performance and reliability.

## ✨ Features

- ⚡ **Lightning Fast**: Sub-10ms startup time, optimized for performance
- 🔒 **Secure**: Safe API key storage using system keyring
- 🌊 **Streaming**: Real-time response streaming for instant feedback
- 💬 **Interactive Mode**: Conversational interface for extended discussions
- 🎨 **Beautiful Output**: Colored, formatted responses with multiple output formats
- 📚 **History Management**: Conversation history with search and export
- 📝 **Templates**: Reusable prompt templates with variable substitution
- 🔧 **Highly Configurable**: Extensive configuration options
- 📦 **Single Binary**: No dependencies, easy to install and distribute

## 🚀 Quick Start

### Installation

#### From Source (Recommended)
```bash
git clone https://github.com/engineers-hub/ask.git
cd ask
cargo build --release
sudo cp target/release/ask /usr/local/bin/
```

#### Using Cargo
```bash
cargo install ask
```

### Setup

1. Set your Anthropic API key:
```bash
ask config set-key YOUR_ANTHROPIC_API_KEY
```

2. Verify the setup:
```bash
ask config show
```

## 📖 Usage

### Basic Usage

```bash
# Simple question
ask "What is Rust programming language?"

# File input
ask -f question.txt

# Pipe input
echo "Explain quantum computing" | ask

# Interactive mode
ask -i
```

### Advanced Usage

```bash
# Custom model and parameters
ask -m claude-3-5-sonnet-20241022 --temperature 0.8 --max-tokens 2000 "Write a poem about AI"

# Different output formats
ask -o json "List 5 programming languages"
ask -o markdown "Compare Python and Rust"

# Verbose output with streaming disabled
ask -v --no-stream "Explain machine learning"

# Continue a conversation
ask -c conversation_id "Follow up question"
```

### Configuration Management

```bash
# View current configuration
ask config show

# Set default model
ask config set-model claude-3-5-sonnet-20241022

# Set max tokens
ask config set-max-tokens 4096

# Set temperature
ask config set-temperature 0.7

# Set output format
ask config set-format markdown

# Reset to defaults
ask config reset

# Show config file path
ask config path
```

### History Management

```bash
# List conversation history
ask history list

# Show specific conversation
ask history show <conversation-id>

# Export conversation
ask history export <conversation-id> -o conversation.json

# Clear history
ask history clear
```

### Template Management

```bash
# List templates
ask template list

# Create template
ask template create code-review "Please review this code: {{code}}"

# Use template
ask template use code-review --var code="fn main() { println!(\"Hello\"); }"
```

## ⚙️ Configuration

Configuration is stored in TOML format. Default locations:
- Linux: `~/.config/ask/ask.toml`
- macOS: `~/Library/Application Support/ask/ask.toml`
- Windows: `%APPDATA%\ask\ask.toml`

Example configuration:
```toml
[api]
model = "claude-3-5-sonnet-20241022"
timeout = 30
max_tokens = 4096
stream = true
temperature = 0.7

[output]
format = "plain"
color = true
verbose = false

[conversation]
save_history = true
max_history_entries = 1000
auto_title = true
```

## 🔐 Security

- API keys are stored securely using the system keyring
- Supports environment variables: `ANTHROPIC_API_KEY` or `ASK_API_KEY`
- Memory-safe Rust implementation
- No data is stored or transmitted beyond Anthropic's API

## 🛠️ Development

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy
```

### Project Structure

```
ask/
├── src/
│   ├── cli/          # Command-line interface
│   ├── client/       # Anthropic API client
│   ├── config/       # Configuration management
│   ├── conversation/ # History management (TODO)
│   ├── template/     # Template system (TODO)
│   ├── output/       # Output formatting (TODO)
│   ├── error.rs      # Error handling
│   ├── lib.rs        # Library root
│   └── main.rs       # Binary entry point
├── tests/            # Integration tests
├── benches/          # Benchmarks
└── docs/            # Documentation
```

## 📊 Performance

- **Startup time**: < 10ms
- **Memory usage**: < 10MB (idle)
- **Binary size**: ~4.3MB (optimized)
- **Async/await**: Full async support with Tokio

## 🤝 Contributing

Contributions are welcome! Please read our contributing guidelines and submit pull requests.

1. Fork the repository
2. Create a feature branch: `git checkout -b my-feature`
3. Commit changes: `git commit -am 'Add feature'`
4. Push to branch: `git push origin my-feature`
5. Submit a pull request

## 📄 License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## 🙏 Acknowledgments

- [Anthropic](https://anthropic.com) for providing the Claude AI API
- The Rust community for excellent libraries and tools
- All contributors and users of this project

---

**Note**: This is an early version with core functionality implemented. Advanced features like conversation history, templates, and enhanced output formatting are in development.

For support, please open an issue on GitHub or contact us at ask@engineers-hub.ltd. 