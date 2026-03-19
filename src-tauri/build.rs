fn main() {
    // Credentials are read at runtime via std::env::var() — no compile-time embedding needed.
    println!("cargo:rerun-if-changed=../.env");
    tauri_build::build()
}
