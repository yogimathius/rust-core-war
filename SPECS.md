# 🏗️ Architectural Guidance for Rust Core War

## 📦 Core Crates & Libraries

### Core Engine

| Purpose                             | Recommended crates                                                                                           |
| ----------------------------------- | ------------------------------------------------------------------------------------------------------------ |
| **Memory + VM state**               | 🛠️ Native Rust primitives (`Vec<u8>`) + maybe [`arrayvec`](https://docs.rs/arrayvec/) for fixed-size buffers |
| **Instruction decoding & encoding** | [`bitflags`](https://docs.rs/bitflags/) or just idiomatic enums with `#[repr(u8)]`                           |
| **Binary file parsing**             | [`binread`](https://docs.rs/binread/) or [`nom`](https://docs.rs/nom/) (if you prefer parser combinators)    |
| **Error handling**                  | Native `Result<T, E>`, plus [`thiserror`](https://docs.rs/thiserror/) for defining error types               |
| **Logging & debugging**             | [`log`](https://docs.rs/log/) + [`env_logger`](https://docs.rs/env_logger/)                                  |

### Assembler

| Purpose                             | Recommended crates                                               |
| ----------------------------------- | ---------------------------------------------------------------- |
| **Lexical Analysis**                | [`logos`](https://docs.rs/logos/) – performant lexer             |
| **Parsing & syntax tree**           | [`pest`](https://docs.rs/pest/) or [`nom`](https://docs.rs/nom/) |
| **Label resolution / symbol table** | Native `HashMap`                                                 |
| **Binary encoding**                 | [`byteorder`](https://docs.rs/byteorder/)                        |

### Game Loop & Scheduling

| Purpose                    | Recommended crates                                                                               |
| -------------------------- | ------------------------------------------------------------------------------------------------ |
| **Timing & rate control**  | [`tokio`](https://tokio.rs/) if you want async features, or just `std::time::Instant`            |
| **Concurrency (optional)** | [`rayon`](https://docs.rs/rayon/) for parallelizable parts                                       |
| **Event loop (optional)**  | [`crossbeam`](https://docs.rs/crossbeam/) if you implement a producer/consumer model for UI & VM |

---

## 🎨 Terminal Visualization

| Purpose                       | Recommended crates                                                                                                 |
| ----------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| **Terminal graphics**         | [`ratatui`](https://github.com/tui-rs-revival/ratatui) (formerly `tui-rs`) – excellent for split views, dashboards |
| **Color / styling**           | [`crossterm`](https://docs.rs/crossterm/) – integrates with `ratatui`                                              |
| **Input (keyboard/mouse)**    | Already built into `crossterm` or use [`termion`](https://docs.rs/termion/)                                        |
| **Layout**                    | Provided by `ratatui` – it supports flexible, responsive layouts                                                   |
| **Animations / trails**       | Custom animations within `ratatui` using timers & layers                                                           |
| **FPS / performance metrics** | Native with `Instant` + `ratatui` widgets                                                                          |

---

## 🧪 Testing & Benchmarking

| Purpose                                 | Recommended crates                              |
| --------------------------------------- | ----------------------------------------------- |
| **Unit tests**                          | Native `#[cfg(test)]`                           |
| **Property-based tests**                | [`proptest`](https://docs.rs/proptest/)         |
| **Golden tests (known inputs/outputs)** | [`insta`](https://docs.rs/insta/) for snapshots |
| **Benchmarking**                        | [`criterion`](https://docs.rs/criterion/)       |

---

## 📝 Documentation & Build

| Purpose                   | Recommended tools                                                          |
| ------------------------- | -------------------------------------------------------------------------- |
| **Docs**                  | Rustdoc + `mdBook` (for user & dev guides)                                 |
| **CLI help**              | [`clap`](https://docs.rs/clap/) – modern, feature-rich command-line parser |
| **Feature flags**         | Use Cargo features (document in `Cargo.toml`)                              |
| **Cross-platform builds** | [`cross`](https://github.com/cross-rs/cross)                               |
| **CI/CD**                 | GitHub Actions + `cargo test`, `cargo clippy`, `cargo fmt`                 |
| **Releases**              | [`cargo-release`](https://docs.rs/cargo-release/)                          |

---

## ⚙️ Implementation Notes

### Memory

- Use a single `Vec<u8>` of size 6144 for VM memory.
- Implement a `Memory` struct with safe `read/write` methods that internally apply modulo addressing (`IDX_MOD`).

### Instructions

- Define an `enum Instruction` with associated data (or separate operand struct).
- Optional: auto-generate opcode tables using `phf` if you want fast lookups.

### Processes

- Each process can be a struct with its PC, registers (`[i32; 16]`), etc.
- You could keep a `VecDeque<Process>` to facilitate round-robin scheduling.

### Visualization

- Build a `ratatui` app with:

  - Memory grid panel (color-coded)
  - Champion info panel
  - Cycle counter and statistics
  - Controls panel

### Concurrency

- Start with single-threaded; later you could separate VM and UI into threads communicating via `crossbeam::channel`.

### Champion File Parsing

- Use `binread` to parse `.cor` files directly into structs (header, code).
- Validate magic number, size, and player metadata.

### Error Handling

- Use `thiserror` for custom VM errors: invalid instruction, out-of-bounds register, file error, etc.

---

## 📅 Suggested Directory Layout

```
corewar-rs/
├── src/
│   ├── main.rs         # CLI entry point
│   ├── lib.rs          # Core library
│   ├── vm/             # Virtual machine core
│   │   ├── memory.rs
│   │   ├── process.rs
│   │   ├── instruction.rs
│   │   ├── scheduler.rs
│   ├── assembler/      # Redcode assembler
│   │   ├── lexer.rs
│   │   ├── parser.rs
│   │   ├── encoder.rs
│   ├── ui/             # Terminal visualization
│   │   ├── app.rs
│   │   ├── components.rs
│   │   ├── input.rs
│   ├── error.rs        # Common error types
├── tests/              # Integration tests
├── benches/            # Benchmarks
├── examples/           # Example champions & usage
├── Cargo.toml
├── README.md
└── docs/
```

---

## 🎯 Next Steps

✅ Create the repository and scaffold directory structure.
✅ Add `clap`, `ratatui`, and `crossterm` to `Cargo.toml`.
✅ Define core types (`Instruction`, `Process`, `Memory`) as plain Rust structs/enums.
✅ Sketch out the VM execution loop.
✅ Write your first integration test: load `.cor`, execute a few cycles, dump memory.
