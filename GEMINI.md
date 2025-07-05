# CLAUDE.md

## 📝 Development Guidelines

### ✅ General Principles

- Use idiomatic **Rust 2024** (`edition = "2024"`) unless specific tooling (e.g., `cross`) forces fallback to 2021.
- Favor **clarity over cleverness**: readable, maintainable code wins over premature optimization.
- Use the latest stable crate versions. Always add dependencies with:
  ```bash
  cargo add <crate>[@version]
  ```

````

rather than editing `Cargo.toml` manually.

### 🧪 Testing & TDD

* Apply **Test-Driven Development (TDD)** wherever practical:

  * Write failing unit tests before implementing features.
  * Keep tests alongside the code or in `/tests`.
  * Use `cargo test` before each commit.

### 📦 Crates & Dependencies

* Use only well-maintained, popular crates.
* Document why any non-obvious crate was chosen.
* Group related dependencies logically in `Cargo.toml` with comments if needed.

### 🔗 Error Handling

* Always prefer `Result<T, E>` over panics in library code.
* Use `thiserror` for custom error types where appropriate.

### 📚 Documentation

* Add doc comments (`///`) for all public functions, structs, enums, and modules.
* If introducing a new module or major feature, update the `README.md` and relevant docs.

### 🚀 Commits & Workflow

* Keep commits focused: one feature or fix per commit.
* Commit messages:

  ```
  feat(vm): implement circular memory addressing
  test(assembler): add label resolution tests
  fix(ui): correct memory grid colors
  ```
* Run:

  ```bash
  cargo fmt
  cargo clippy
  cargo test
  ```

  before pushing.

### 🖥️ CLI & UX

* Ensure CLI tools (`corewar`, `asm`) output helpful errors and `--help` information.
* Make default behaviors safe and predictable.

### 📂 Project Structure

* Keep modules organized and small; avoid monoliths.
* Prefer descriptive module and file names.

---

## 🤖 Notes for Agents

* Be verbose in your reasoning while drafting PRs or code suggestions.
* Annotate non-trivial code with comments explaining intent.
* If uncertain, propose options with pros & cons for human review.
````
