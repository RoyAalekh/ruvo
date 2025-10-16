# Ruvo - Fast, Lightweight Virtual Environment Manager

**Ruvo** is a high-performance, lightweight Rust-based virtual environment manager for Python. It simplifies creating, managing, and switching between virtual environments with speed and efficiency. Designed for developers who value performance and simplicity, Ruvo provides a streamlined experience for Python environment management.

## Features

- **High Performance** - Built in Rust for optimal speed and efficiency
- **Multiple Environment Support** - Compatible with `venv`, `poetry`, and `uv`
- **Simple Command Interface** - Intuitive CLI for quick environment management
- **Minimal Dependencies** - Lightweight design without unnecessary bloat
- **Python Version Control** - Create environments with specific Python versions
- **Instant Operations** - Fast activation, creation, and deletion of environments

## Installation

Install Ruvo directly from GitHub:

```bash
pip install git+https://github.com/RoyAalekh/ruvo.git
```

## Usage

### Create a New Environment

```bash
ruvo create my-env --env-type venv
ruvo create poetry-env --env-type poetry --python-version 3.11
```

### List Available Environments

```bash
ruvo list
```

### Activate an Environment

```bash
ruvo activate my-env
```

### Delete an Environment

```bash
ruvo delete my-env
```

## Why Choose Ruvo?

- **Performance**: Rust-based implementation outperforms traditional environment managers
- **Developer Experience**: Simple commands, intuitive design, and easy setup
- **Cross-Platform**: Seamless operation on Windows, macOS, and Linux
- **Versatility**: Support for multiple environment types (`venv`, `poetry`, `uv`)
- **Efficiency**: Minimal resource usage and fast execution times

## Command Reference

| Command | Description | Example |
|---------|-------------|----------|
| `create` | Create new virtual environment | `ruvo create myenv --env-type venv` |
| `list` | List all environments | `ruvo list` |
| `activate` | Activate environment | `ruvo activate myenv` |
| `delete` | Remove environment | `ruvo delete myenv` |

## Supported Environment Types

- **venv** - Python's built-in virtual environment
- **poetry** - Poetry-managed environments
- **uv** - Ultra-fast Python package installer environments

## Requirements

- Python 3.7+
- Rust (for building from source)
- Compatible with Windows, macOS, and Linux

## Contributing

Contributions are welcome! To contribute:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

## Support

- [GitHub Issues](https://github.com/RoyAalekh/ruvo/issues) - Report bugs and request features
- [Discussions](https://github.com/RoyAalekh/ruvo/discussions) - Share ideas and get help

## License

MIT License - see LICENSE file for details.

---

Speed up your Python development workflow with Ruvo's efficient environment management.
