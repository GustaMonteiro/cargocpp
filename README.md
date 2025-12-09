# CargoCpp

A Rust-inspired C++ project management tool that simplifies the creation and management of C++ projects using CMake and vcpkg.

## Overview

CargoCpp aims to bring the simplicity and convenience of Rust's Cargo tool to C++ development. It provides a streamlined workflow for creating, building, and managing C++ projects with integrated dependency management through vcpkg.

## Features

- 🚀 **Quick Project Creation**: Initialize new C++ projects with a single command
- 🏗️ **CMake Integration**: Automatic CMake configuration and build management
- 📦 **Dependency Management**: Easy dependency addition (future vcpkg integration)
- 🔧 **Multiple C++ Standards**: Support for C++14, C++17, C++20, and C++23
- 🧹 **Build Management**: Clean, build, and run commands similar to Cargo

## Installation

### Prerequisites

- Rust (for building the tool)
- CMake
- Git
- A C++ compiler (GCC, Clang, or MSVC)

### Build from Source

```bash
git clone https://github.com/GustaMonteiro/cargocpp.git
cd cargocpp
cargo build --release
```

The binary will be available at `target/release/cargocpp` (or `target/release/cargocpp.exe` on Windows).

## Usage

### Creating a New Project

```bash
cargocpp new my_project
```

Create a project with a specific C++ standard:

```bash
cargocpp new my_project --std 23
```

Supported C++ standards: 14, 17, 20, 23 (default: 20)

### Building the Project

```bash
cargocpp build
```

Clean build (removes build artifacts first):

```bash
cargocpp build --clean
```

### Running the Project

```bash
cargocpp run
```

Run in quiet mode (suppress build output):

```bash
cargocpp run --quiet
```

### Managing Dependencies

Add dependencies to your project:

```bash
cargocpp add library1 library2
```

### Cleaning Build Artifacts

```bash
cargocpp clean
```

## Project Structure

When you create a new project with `cargocpp new project_name`, the following structure is generated:

```
project_name/
├── CMakeLists.txt          # CMake configuration
├── src/
│   └── main.cpp           # Main source file
├── include/
│   └── pch.h             # Precompiled header
└── .git/                 # Git repository (initialized automatically)
```

## Commands Reference

| Command | Description | Options |
|---------|-------------|---------|
| `new <name>` | Create a new C++ project | `--std <version>`: Set C++ standard (14, 17, 20, 23) |
| `build` | Build the current project | `--clean`: Clean before building |
| `run` | Build and run the project | `--quiet`, `-q`: Suppress build output |
| `add <deps...>` | Add dependencies to the project | Multiple dependency names |
| `clean` | Remove all build artifacts | None |

## Roadmap

- [ ] **vcpkg Integration**: Automatic dependency management through vcpkg
- [ ] **Project Templates**: Support for different project templates (library, executable, etc.)
- [ ] **Configuration Files**: Project-specific configuration similar to Cargo.toml
- [ ] **Cross-platform Builds**: Enhanced support for different platforms and toolchains
- [ ] **Package Publishing**: Ability to publish and share C++ packages
- [ ] **Testing Framework**: Integrated testing support
- [ ] **Benchmarking**: Built-in benchmarking capabilities

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Inspiration

This project is inspired by Rust's Cargo package manager, aiming to bring similar ease of use and functionality to C++ development.