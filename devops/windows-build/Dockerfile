FROM mcr.microsoft.com/windows/servercore:ltsc2022

SHELL ["powershell", "-Command", "$ErrorActionPreference = 'Stop'; $ProgressPreference = 'SilentlyContinue';"]

# Install Chocolatey
RUN Set-ExecutionPolicy Bypass -Scope Process -Force; \
    [System.Net.ServicePointManager]::SecurityProtocol = \
        [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; \
    iex ((New-Object System.Net.WebClient).DownloadString( \
        'https://community.chocolatey.org/install.ps1'))

# Install MSVC build tools (required by Rust's default windows-msvc toolchain),
# Node.js LTS, pnpm, and just
RUN choco install visualstudio2022buildtools \
        --package-parameters "--add Microsoft.VisualStudio.Workload.VCTools --includeRecommended --quiet" \
        -y; \
    choco install nodejs-lts pnpm just -y

# Install Rust stable (MSVC toolchain) and persist cargo to the machine PATH
RUN Invoke-WebRequest -Uri 'https://win.rustup.rs/x86_64' -OutFile C:\rustup-init.exe; \
    C:\rustup-init.exe -y --default-toolchain stable-x86_64-pc-windows-msvc --no-modify-path; \
    Remove-Item C:\rustup-init.exe; \
    [Environment]::SetEnvironmentVariable( \
        'PATH', \
        "$env:USERPROFILE\.cargo\bin;$([Environment]::GetEnvironmentVariable('PATH', 'Machine'))", \
        'Machine')

# Install Tauri CLI (cargo is now on the machine PATH)
RUN cargo install tauri-cli --locked

CMD ["powershell"]
