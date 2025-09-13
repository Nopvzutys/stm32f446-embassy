# Build
```bash
cargo build
```

# Debug
```bash
cargo run
```

# Release Mode
```bash
cargo run --release
```

# Check Binary Size
```bash
#Install
cargo install cargo-bloat

#See largest functions from my code
cargo bloat --release -n 10

#See largest dependencies
cargo bloat --release --crates
```