# Crabstat — Rust Learning Plan

A blazing fast CLI tool that displays system information.

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
- [ ] OS name and version (`std::env::consts::OS`, `std::env::consts::ARCH`)
- [ ] Hostname
- [ ] Username
- [ ] Current directory (`std::env::current_dir()`, learn `PathBuf`)
- [ ] IP address (network programming or subprocess)

### Concepts to learn
- **Structs**: Define custom types, `impl` blocks for methods
- **Dependencies**: Adding crates to `Cargo.toml` (`sysinfo`, `clap`)
- **Iterators**: Zero-cost, chainable `.map().filter()` compiled away
- **Lifetimes**: The `'a` annotation — will encounter naturally
- **Async**: `tokio`, `async`/`.await`, `join!` for parallel info gathering

### Key crates
- `sysinfo` — cross-platform system info (CPU, RAM, disks, processes)
- `clap` — CLI argument parsing
- `colored` / `owo-colors` — terminal colors

## Phase 3: Make It Pretty

- [ ] Formatted, colored output (like `neofetch` / `fastfetch`)
- [ ] Implement `Display` trait for custom types
- [ ] Split code into modules: `src/info/`, `src/display/`

### Concepts to learn
- **Traits**: Implement `Display` (like `toString()`)
- **Modules**: Organize code into files and directories
- **String formatting**: `format!`, `write!` macros

## Phase 4: Testing

- [ ] Unit tests (in-file `#[cfg(test)]` modules)
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
- [ ] Learn `Pin`, `Send`, `Sync`

### Key difference from TS/Node
No built-in async runtime — you choose one (`tokio` is standard).
`#[tokio::main]` annotates your main function to set up the runtime.

## Phase 6: Polish & Ship

- [ ] `cargo install` support
- [ ] Cross-compilation (Linux/Windows from macOS)
- [ ] GitHub Actions release pipeline
- [ ] Publish to crates.io with `cargo publish`

## Recommended Resources

- **The Rust Book**: `rustup doc --book`
- **Rust by Example**: learn from runnable code
- **Exercism Rust track**: practice problems
- **"From JavaScript to Rust"**: tailored for JS/TS developers
