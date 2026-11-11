# Contributing to SamuSoporte

Thank you for your interest in contributing to **SamuSoporte**! We welcome bug reports, feature suggestions, architectural improvements, and code contributions from the global community.

---

## 1. Code of Conduct

All contributors and maintainers are expected to follow our [Code of Conduct](CODE_OF_CONDUCT.md). Please treat all participants with respect and professional courtesy.

---

## 2. Getting Started & Development Environment

### Prerequisites
- **Operating System:** Windows 10 (Build 19041+) or Windows 11 (22H2+ recommended).
- **.NET SDK:** .NET 10 LTS SDK (`dotnet --version` >= 10.0.100).
- **Rust Toolchain:** Stable Rust with `x86_64-pc-windows-msvc` target (`rustup default stable`).
- **Visual Studio / Build Tools:** Visual Studio 2022 / C++ Build Tools (with Windows SDK 10.0.26100.0).
- **Git:** Git for Windows.

### Cloning the Repository
```powershell
git clone https://github.com/saghet/SamuSoporte.git
cd SamuSoporte
```

---

## 3. Building the Project

### Building the Native Desktop Client (.NET 10 / WPF-UI)
```powershell
dotnet build src/OpenSupport.Desktop/OpenSupport.Desktop.csproj -c Release
```

### Building the Core Rust Crates & Services
```powershell
cargo build --workspace --release
```

---

## 4. Running Tests

We maintain strict regression and security verification. All tests must pass before submitting a Pull Request:

```powershell
# Run all Rust workspace tests (Crypto, Codec, Audit, Input, Transfer, Diagnostics)
cargo test --workspace

# Run .NET solution tests
dotnet test
```

---

## 5. Branching Strategy & Pull Requests

1. **Fork & Branch:** Create a branch prefixed with the change type:
   - `feat/feature-name`
   - `fix/bug-description`
   - `sec/hardening-improvement`
   - `docs/documentation-update`
2. **Commit Style:** Use Conventional Commits (`feat: ...`, `fix: ...`, `chore: ...`).
3. **No Secret Leaks:** Verify that no credentials, tokens, or local `.env` files are tracked.
4. **Pull Request Checklist:**
   - Link the relevant GitHub Issue.
   - Confirm that all unit and security integration tests pass.
   - Include screenshots or diagnostic logs if proposing UI or performance changes.
