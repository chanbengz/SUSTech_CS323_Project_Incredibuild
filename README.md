<div align=center>

# SUSTech CS323 Project: Incredibuild

SPL(SUSTech Programming Language) Compiler in Rust

</div>

> [!WARNING]
> This project is still on the fly.

## Features

- [x] SPL Grammar:
    - [x] Lexer & Parser
    - [x] Semantic Analyser
    - [ ] Macro
- [ ] LLVM IR Generation
- [ ] Optimization
    - [ ] SSA
- [ ] Advanced
    - [ ] Lifetime Analysis

## Getting Started

At first, you need to setup the environment by installing Rust, Cargo and LLVM, please follow instruction in [development](#development) section. 
Then you can build the project by running

```
cargo build
```

and emit the assembly code by running

```bash
cargo run /path/to/input.spl -o /path/to/output.S
```

or check the LLVM IR by running

```bash
cargo run /path/to/input.spl --llvm-ir -o /path/to/output.ll
```

Finally, to execute the program, you can use LLVM's interpreter or compile the assembly

```bash
cc /path/to/output.S -o /path/to/output
./output

# or
lli /path/to/output.ll
```

> [!NOTE]
> Some of the functionalities are weird in release mode.

See `target/debug/incredibuild -h` for more options.

## File structure

```
.
├── Cargo.toml # Binding of submodules
├── Dockerfile # Build the environment for the project
├── docs
│   ├── cs323-project-tutorial1.pdf
│   ├── cs323-project-tutorial2.pdf
│   ├── cs323-project-tutorial3.pdf
│   ├── cs323-project-tutorial4.pdf
│   ├── midterm-project-check.pdf
│   ├── slides # Presentation slides
│   ├── syntax.txt
│   └── token.txt
├── LICENSE
├── README.md
├── src
│   ├── analyser
│   │   ├── Cargo.toml # Define your dependencies in the sub-project
│   │   └── src # In lib.rs please pub mod all the crates.
│   ├── ast
│   │   ├── Cargo.toml
│   │   └── src
│   ├── irgen
│   │   ├── Cargo.toml
│   │   └── src
│   ├── lexer
│   │   ├── Cargo.toml
│   │   └── src
│   ├── main.rs
│   └── parser
│       ├── build.rs # lalrpop parser generator
│       ├── Cargo.toml
│       └── src
└── test
    ├── phase1 # Lexical & Syntax
    ├── phase2 # Semantic
    ├── phase3 # LLVM IR Generation
    ├── phase4 # Binary Generation
    ├── test_0_r00.spl # Minimal testcase
    ├── test_0_r00.ll  # LLVM IR of the minimal testcase
    └── test_0_r00.out # Sample output
```


## Development

Setting up environment might be a stuggling process, you can use `docker` to build up the environment.

```bash
docker build -t incredibuild .
```

You can also choose to download the image we have already built from Docker Hub.

```bash
docker pull jaredanjerry/incredibuild:20241106 # Approximately 4.5GB
```

After building the project, you can set up the project by running an iteractive shell in the container. (Please mount the project root path to `/incredibuild`)

```bash
cd <project-root-path>
docker run -it --rm -v $(pwd):/incredibuild incredibuild
cargo run
```

### "Bare Metal" Mac

If you're using Mac, make sure you have Rust and Cargo installed. Install LLVM environment by

```bash
brew install llvm # or use the package manager you prefer

# find where llvm is installed, for example
export LLVM_SYS_170_PREFIX=/opt/homebrew/Cellar/llvm/19.1.2 # pretend to be LLVM@17 :)
```

### Testing

You can run test in each crate by using `cargo test` in the root folder. For example

```bash
cd src/lexer
cargo test
```

## Issue
If you face network issue, luckily you can find two ways out.

1. Modify `/etc/systemd/system/docker.service.d/http-proxy.conf` to add proxy.

    ```conf
    [Service]
    Environment="HTTP_PROXY=http://your-proxy:port"
    Environment="HTTPS_PROXY=http://your-proxy:port"
    ```

    Then restart docker service.

    ```bash
    sudo systemctl daemon-reload
    sudo systemctl restart docker
    ```

2. Add Registry Mirror in `/etc/docker/daemon.json`.

    ```json
    {
        "registry-mirrors": ["<your-mirror>"]
    }
    ```

## Reference

Our presentation slides are available [here](https://chanbengz.github.io/slides/compilers-demo) (Chinese)

Acknowledgement:
- [llvm@17](https://llvm.org/)
- [lalrpop](https://github.com/lalrpop/lalrpop)
- [Logos](https://github.com/maciejhirsz/logos)
- [inkwell](https://github.com/TheDan64/inkwell)
