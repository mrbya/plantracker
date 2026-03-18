fn main() {
    println!("cargo:rerun-if-env-changed=VITE_AZURE_CLIENT_ID");
    println!("cargo:rerun-if-env-changed=VITE_AZURE_TENANT_ID");
    tauri_build::build()
}
