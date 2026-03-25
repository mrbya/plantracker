# Phase 0 — Project Scaffold

## 0.1 Initialize Tauri + Svelte project (Done)

```bash
pnpm create tauri-app plantracker \
  --template svelte-ts \
  --manager pnpm \
  --tauri-version 2
cd plantracker
```

## 0.2 Install frontend dependencies

```bash
pnpm add -D @types/node
# No runtime UI framework — vanilla Svelte only
```

## 0.3 Install Rust crates

In `src-tauri/Cargo.toml`, add:

```toml
[dependencies]
tauri = { version = "2", features = ["protocol-asset"] }
tauri-plugin-store = "2"
tauri-plugin-shell = "2"
tauri-plugin-oauth = "2"          # localhost redirect server
tauri-plugin-dialog = "2"         # Save file dialogs (CSV export)
sqlx = { version = "0.8", features = ["sqlite", "runtime-tokio-rustls", "chrono", "macros"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
reqwest = { version = "0.12", features = ["json", "rustls-tls"], default-features = false }
keyring = "2"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1", features = ["v4"] }
anyhow = "1"
base64 = "0.22"
sha2 = "0.10"
rand = "0.8"
csv = "1"
```

## 0.4 Set up environment config

Create `.env.example`:
```env
VITE_AZURE_CLIENT_ID=
VITE_AZURE_TENANT_ID=common
```

Create `src-tauri/build.rs` to forward env vars to Rust at compile time:
```rust
fn main() {
    println!("cargo:rerun-if-env-changed=VITE_AZURE_CLIENT_ID");
    tauri_build::build()
}
```

## 0.5 Configure Tauri capabilities

In `src-tauri/capabilities/default.json`, enable:
- `core:default`
- `shell:allow-open` (system browser)
- `store:default`
- `dialog:default`
- `oauth:default`

## Verification checklist
- [x] `cargo tauri dev` launches without errors
- [x] Svelte HMR works (edit `App.svelte`, see instant update)
- [x] `.env` variables accessible as `import.meta.env.VITE_AZURE_CLIENT_ID`
