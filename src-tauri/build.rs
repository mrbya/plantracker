fn main() {
    // Forward Azure AD env vars so env!() works in source code.
    // Defaults to empty string when not set — app compiles but auth will fail without credentials.
    let client_id = std::env::var("VITE_AZURE_CLIENT_ID").unwrap_or_default();
    let tenant_id = std::env::var("VITE_AZURE_TENANT_ID").unwrap_or_default();
    println!("cargo:rustc-env=VITE_AZURE_CLIENT_ID={client_id}");
    println!("cargo:rustc-env=VITE_AZURE_TENANT_ID={tenant_id}");
    println!("cargo:rerun-if-env-changed=VITE_AZURE_CLIENT_ID");
    println!("cargo:rerun-if-env-changed=VITE_AZURE_TENANT_ID");
    tauri_build::build()
}
