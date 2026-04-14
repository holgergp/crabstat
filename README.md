# crabstat
This a blazing fast System Info tool. Written in Rust. It's blazing fast.

I am learning a little bit of rust. And I need something concrete to work on. I want to build a CLI tool. Never done that before. So this tool gathers some system info. Yes I know: Available elsewhere. but it might come handy for me.

## Usage

```bash
cs              # show all system info
cs --shell      # show only shell info
cs --net        # show only network info
cs --os         # show only OS info
cs --help       # show help
```

## Building

```bash
cargo build --release    # optimized binary in target/release/cs
cargo run                # compile and run (debug mode)
```

Supports macOS and Linux. Has platform-specific detection for OS version.

## Installation

### From GitHub Releases

Download the latest binary for your platform from the [Releases](../../releases) page.

```bash
# macOS (Apple Silicon)
chmod +x cs-macos-arm64 && mv cs-macos-arm64 /usr/local/bin/cs

# Linux (x86_64)
chmod +x cs-linux-amd64 && mv cs-linux-amd64 /usr/local/bin/cs
```

### From source

```bash
cargo install --path .
```

## Releasing

To create a new release, tag and push:

```bash
git tag v0.1.0
git push --tags
```

This triggers the GitHub Actions release workflow, which builds binaries for macOS (ARM + x86) and Linux (x86), and attaches them to a GitHub Release.

## Example output

```
╔══════════════════╗
║  🦀 crabstat     ║
╚══════════════════╝
Shell          : fish (fish, version 4.6.0)
Current Dir    : /Users/user/projects/crabstat
IP             : 192.168.XXX.XX
OS Name        : macos
Architecture   : aarch64
Kernel Version : 25.4.0
OS Version     : 26.4.1
Hostname       : my-machine.local
User           : user
```