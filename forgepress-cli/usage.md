### Verification: Compilation and Usage

You can compile this crate, and utilize it as a developer CLI locally:

#### 1. Compile the tool
```bash
cargo build --package forgepress-cli --release
```

#### 2. Create your first administrative account
```bash
./target/release/forgepress-cli create-admin --username admin --email admin@yoursite.com --password secure_pass
```

#### 3. Scaffold a new visual theme
```bash
./target/release/forgepress-cli install-theme --name modern-rust-grid
```

#### 4. Clear the memory caches dynamically
```bash
# Purge everything
./target/release/forgepress-cli clear-cache --target all

# Purge a specific page slug only
./target/release/forgepress-cli clear-cache --target about-us
```