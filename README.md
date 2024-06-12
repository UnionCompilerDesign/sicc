# SICC Starter Code

## Overview
The sicc Starter Code repository hosts the initial codebase for the Simple Instructional C99 Compiler (sicc), designed for undergraduate-level courses in compiler design. This repository provides the foundational structure and configurations to complete the assignments found in the `docs` folder. 

## Project Structure
- `common/`: Contains definitions for Abstract Syntax Trees (`AST`) and shared modules.
- `integration/`: Integrates functionality from `common` and `sts` into the `Module` type.
- `ir/`: Handles conversion of `Module` instances into pre-compiled LLVM modules in parallel.
- `lexer/`: Transforms source code into tokenized instances.
- `parser/`: Constructs an AST from tokenized input.
- `src/`: Main driver and entry point for the compiler.
- `sts/`: Generates a Symbol Table Stack (STS) from an AST.
- `tests/`: Conducts end-to-end testing, processing `.c` files through `src` and validating the resulting LLVM modules.

## Getting Started
### Prerequisites
- Compatible with major Linux distributions and MacOS.
- Requires `llvm-17`, `llvm-17-dev`, `llvm-17-tools`, `clang-17`, and `llvm_sys`.
- Rust development tools: `rustc` and `cargo`.
Alternatively:
- Docker CLI and Docker Daemon for container-based development.

### Installation
To clone the repository and set up the starter code, run:
```rust
git clone git@github.com:UnionCompilerDesign/sicc.git
cd sicc
```
To install the Docker CLI and the Docker Daemon, follow the instructions for your operating system:
#### macOS
    1. Download Docker Desktop for macOS from https://hub.docker.com/.
    2. Open the downloaded .dmg file and drag the Docker icon to your Applications folder.
    3. Run Docker from your Applications folder. 

### Ubuntu
```bash
sudo apt update
sudo apt install apt-transport-https ca-certificates curl software-properties-common
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo apt-key add -
sudo add-apt-repository "deb [arch=amd64] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable"
sudo apt update
sudo apt install docker-ce
```

### Usage
1. See the `docs` repository for an overview of the project and the assignment documents.
2. You can then build each part of the project using the `Dockerfile` found at the root of each directory. The bottom of each `Dockerfile` has a build and run command. After running the build and run commands:
```bash
cd sicc/<directory_name>
cargo build
cargo test
```
3. You can also build the entire project using the `Dockerfile` found at the root. After running the build and run commands:
```bash
cd sicc
cargo build --all
cargo test --all
```

### How to Contribute
Contributions are welcome! Please refer to the CONTRIBUTING.md file in `docs` for guidelines on how to submit patches and bug reports.

### License
Distributed under the MIT License. See the LICENSE file for more details.

### Acknowledgements
Special thanks to Professor Aaron Cass of Union College for his guidance and expertise throughout the development of this project.

### Contact
For any inquiries, contact Caleb L'Italien at litaliencaleb@gmail.com.
