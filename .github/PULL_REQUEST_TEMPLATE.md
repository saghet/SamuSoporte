## Description of Changes
Please provide a clear and concise summary of the changes made in this Pull Request.

## Type of Change
- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Security hardening or cryptographic improvement
- [ ] Performance optimization
- [ ] Documentation update
- [ ] CI/CD or build pipeline enhancement

## Component(s) Affected
- [ ] `src/OpenSupport.Desktop` (Native WPF-UI client)
- [ ] `src/OpenSupport.Service` / `apps/opensupport-service` (Windows Background Service)
- [ ] `apps/opensupport-server` (Relay & Rendezvous Server)
- [ ] `crates/opensupport-crypto` (Noise Protocol & E2EE)
- [ ] `crates/opensupport-codec` / `opensupport-input` / `opensupport-transfer`
- [ ] Other: 

## Verification Checklist
- [ ] My code follows the code style and guidelines of this project.
- [ ] I have executed `cargo test --workspace` and all unit/integration tests pass.
- [ ] I have executed `dotnet build -c Release` and the project compiles with 0 errors.
- [ ] I have not committed any passwords, API keys, credentials, or sensitive secrets.
- [ ] I have updated the documentation accordingly if this change introduces user-facing features.
