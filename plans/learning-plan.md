# Crabstat — Rust Learning Plan

A blazing fast CLI tool that displays system information.
GitHub repo: hosted on user's personal GitHub.

## Session Context

**Current state:** Phase 3 in progress. Colored output, modular code, custom enum with impl block. One external dependency (`colored`). CI pipeline active. Pre-commit hooks via `cargo-husky`.

**Current code structure:**
- `src/main.rs` — mod declarations + main (3 lines)
- `src/system.rs` — `SystemInfo` struct, `get_system_info()` orchestrator
- `src/shell.rs` — `ShellInfo` struct, shell name/version detection, unit tests
- `src/os.rs` — `OsInfo` struct, OS/arch/kernel/version with conditional compilation, unit tests
- `src/network.rs` — `NetworkInfo` struct, IP address (UDP socket), hostname, uses `InfoValue`
- `src/user.rs` — username, current directory
- `src/types.rs` — `InfoValue` enum with `impl` block (`from_result` constructor)
- `src/display.rs` — `print_system_info`, colored output, handles `InfoValue` variants
- `.github/workflows/ci.yml` — CI pipeline (fmt, clippy, build, test, cargo run)

**Teaching approach:** User writes all code. I explain concepts, review code, help debug compiler errors. Do not write implementation code unless explicitly asked.

**What the user has learned so far:**
- Basic syntax: `fn`, `let`, `println!`, `match`
- `String` vs `&str` (owned vs borrowed)
- Ownership and moves (hit "value used after move" and fixed it)
- Borrowing with `&` (refactored functions from `String` to `&str` params)
- `Result` and `Option` handling: `unwrap`, `unwrap_or`, `unwrap_or_else`, `match`
- The `?` operator for propagating errors
- `Result<T, E>` as return type
- Closures: `|x| expr` syntax
- `std::process::Command` for subprocesses
- `String::from_utf8_lossy` for bytes → string
- `std::env::current_dir()` and `PathBuf`
- `std::net::UdpSocket` for local IP detection
- `::` (path separator) vs `.` (method call)
- Single quotes (char) vs double quotes (string)
- `cargo run`, `cargo build`, `cargo fmt`, `cargo clippy`
- Implicit returns (last expression without semicolon)
- Structs: defining, nesting, field shorthand
- Conditional compilation with `#[cfg(target_os)]`
- `std::fs::read_to_string` for file reading
- Iterator chaining: `.lines()`, `.find()`, `.map()`, `.starts_with()`, `.strip_prefix()`
- Naming conventions: `snake_case` functions, `CamelCase` types (not `ALLCAPS`)
- When to use `'static` lifetimes vs just `.to_string()`
- Modules: `mod`, `pub`, `use crate::`, sibling module access
- Visibility: everything private by default, `pub` on structs, fields, and functions
- Enums with data: defining `InfoValue` with `Available(String)` / `Unavailable(String)`
- `impl` blocks: methods on enums/structs (`from_result` constructor)
- Explicit lifetimes: `'a` annotations when compiler can't infer them
- `matches!` macro
- External dependencies: adding crates with `cargo add`
- Pre-commit hooks with `cargo-husky`
- Traits: `impl Display for T`, `write!` macro, `&mut` references
- `Display` gives `.to_string()` for free

## Concepts Map (Rust ↔ TypeScript/JVM)

| Rust | TypeScript / JVM equivalent |
|---|---|
| `cargo` | `npm` + `tsc` + bundler |
| `Cargo.toml` | `package.json` |
| `Cargo.lock` | `package-lock.json` |
| crate (library) | npm package / Maven artifact |
| `crates.io` | npmjs.com / Maven Central |
| `mod` | ES modules / Java packages |
| `trait` | interface (like Haskell typeclasses) |
| `enum` | discriminated union / sealed class (with data!) |
| `Result<T, E>` | like `Either` / errors are values, no exceptions |
| `Option<T>` | like `T \| null` but compiler-enforced |
| `&T` / `&mut T` | immutable/mutable reference (no GC!) |
| ownership | **nothing like it in TS/JVM** |
| `async`/`.await` | `async`/`await` |
| `Future` | `Promise` |
| `tokio` runtime | Node's event loop (but explicit) |
| `tokio::join!` | `Promise.all()` |
| `tokio::select!` | `Promise.race()` |
| `tokio::spawn` | lightweight green thread |

## Phase 0: Setup ✅

- [x] Install Rust via `rustup`
- [x] Create project with `cargo init`
- [x] Set up GitHub repo
- [x] Set up CI (GitHub Actions: build, fmt, clippy, test)

## Phase 1: Hello Shell ✅

- [x] Print current shell name from `SHELL` env var
- [x] Extract shell name from path using `rsplit`
- [x] Get shell version via `std::process::Command`
- [x] Handle errors with `match` on `Result`
- [x] Extract logic into functions with proper borrowing (`&str`)

### Concepts learned
- `fn main()`, `let`, `println!`, `String` vs `&str`
- `std::env::var()` returns `Result`, not a plain value
- `unwrap()`, `unwrap_or()`, `unwrap_or_else()`
- `match` as an expression (exhaustive, returns a value)
- `std::process::Command` for spawning subprocesses
- `String::from_utf8_lossy` for bytes → string
- Ownership and borrowing: moves, `&` references, `&str` vs `&String`
- Closures: `|x| expr` syntax

## Phase 2: Expand System Info 🔜

### Features to add
- [x] OS name and version (`std::env::consts::OS`, `std::env::consts::ARCH`)
- [x] Hostname
- [x] Username
- [x] Current directory (`std::env::current_dir()`, learn `PathBuf`)
- [x] IP address (`std::net::UdpSocket` trick, cross-platform, no dependencies)

### Concepts to learn
- ~~**Structs**: Define custom types~~ ✅ (SystemInfo, ShellInfo, OsInfo)
- **`impl` blocks**: Methods on structs
- **Dependencies**: Adding crates to `Cargo.toml` (`sysinfo`, `clap`)
- **Iterators**: Zero-cost, chainable `.map().filter()` compiled away
- **Lifetimes**: The `'a` annotation — will encounter naturally
- **Async**: `tokio`, `async`/`.await`, `join!` for parallel info gathering

### CLI arguments (planned)
- [ ] Add `clap` for argument parsing
- [ ] Flags to show only specific info (e.g., `--shell`, `--ip`, `--os`)
- [ ] Learn derive macros through clap's `#[derive(Parser)]`

### Key crates
- `sysinfo` — cross-platform system info (CPU, RAM, disks, processes)
- `clap` — CLI argument parsing
- `colored` / `owo-colors` — terminal colors

## Phase 3: Make It Pretty

- [x] Formatted, colored output (like `neofetch` / `fastfetch`)
- [ ] Dark and light terminal mode support
- [ ] Implement `Display` trait for custom types
- ~~Split code into modules~~ ✅ (done in Phase 2)

### Concepts to learn
- **Traits**: Implement `Display` (like `toString()`)
- **Modules**: Organize code into files and directories
- **String formatting**: `format!`, `write!` macros

## Phase 4: Testing

- [x] Unit tests (in-file `#[cfg(test)]` modules)
- [x] Edge case tests (empty strings, missing slashes, whitespace, multiline output)
- [ ] Integration tests (`tests/` directory)
- [ ] Async tests with `#[tokio::test]`

### How testing works in Rust
- Unit tests live **in the same file** inside `#[cfg(test)]`
- Integration tests live in `tests/` directory
- `cargo test` finds and runs everything
- No separate test framework needed

## Phase 5: Async

- [ ] Add `tokio` runtime
- [ ] Gather system info concurrently with `tokio::join!`
- [ ] Add spinners (`indicatif`) while gathering info
- [ ] Learn `Pin`, `Send`, `Sync`

### Key difference from TS/Node
No built-in async runtime — you choose one (`tokio` is standard).
`#[tokio::main]` annotates your main function to set up the runtime.

## Phase 6: Polish & Ship

- [ ] `cargo install` support
- [ ] Cross-compilation (Linux/Windows from macOS)
- [ ] GitHub Actions release pipeline
- [ ] Publish to crates.io with `cargo publish`

## Rust Concepts Still to Cover

### Must — core language, will encounter everywhere
- [x] **Enums with data** — `InfoValue` enum with `Available`/`Unavailable` variants
- [x] **Traits** — implemented `Display` for `InfoValue` and `ShellInfo`, understand `write!`, `&mut`, `fmt::Result`
- [ ] **Generics** — `fn process<T>(item: T)`, you've seen `Result<T, E>` but haven't written your own
- [ ] **Error handling (proper)** — `anyhow`/`thiserror` crates, custom error types, replacing `e.to_string()` patterns
- [x] **Lifetimes (explicit)** — encountered with `value_or` returning from two sources
- [ ] **Derive macros** — `#[derive(Debug, Clone, PartialEq)]`, code generation at compile time
- [ ] **Iterators (deeper)** — write your own iterator, `.collect()` into different types

### Optional — useful but can wait
- [ ] **Smart pointers** — `Box`, `Rc`, `Arc` for complex ownership
- [ ] **Enums as state machines** — model app states so invalid states are unrepresentable
- [ ] **Serialization with `serde`** — JSON output for piping to other tools
- [ ] **Closures (deeper)** — `Fn`, `FnMut`, `FnOnce` traits, capturing variables
- [ ] **Pattern matching (advanced)** — destructuring, guards, nested patterns
- [ ] **Unsafe Rust** — raw pointers, FFI, when safe Rust isn't enough

## Recommended Resources

- **The Rust Book**: `rustup doc --book`
- **Rust by Example**: learn from runnable code
- **Exercism Rust track**: practice problems
- **"From JavaScript to Rust"**: tailored for JS/TS developers
