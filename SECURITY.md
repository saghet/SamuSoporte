# Security Policy

SamuSoporte takes cybersecurity, cryptographic verification, and operational resilience seriously. We welcome responsible disclosures from security researchers, penetration testers, and contributors.

---

## 1. Supported Versions

Security patches and critical hardening updates are actively provided for the following releases:

| Version | Status | Security Support |
| :--- | :--- | :--- |
| **0.1.x** | **Active (Current Preview)** | ✅ Supported |
| < 0.1.0 | Deprecated | ❌ Unsupported |

---

## 2. Reporting a Vulnerability

If you discover a security vulnerability in SamuSoporte (such as cryptographic flaws in the Noise Protocol implementation, Windows Named Pipe privilege escalation, denial of service in relay handling, or memory corruption in Rust crates), **please DO NOT open a public GitHub issue.**

### How to Report Privately:
1. **GitHub Security Advisory (Preferred):**
   - Submit a report via the [GitHub Security Advisory form](https://github.com/saghet/SamuSoporte/security/advisories/new).
2. **Include Key Technical Details:**
   - Affected component (e.g. `opensupport-crypto`, `OpenSupport.Desktop`, `opensupport-service`, `opensupport-server`).
   - Detailed step-by-step reproduction guide or Proof of Concept (PoC).
   - Potential impact (e.g. unauthorized control, privilege escalation, man-in-the-middle).
   - Suggested remediation or patch (if available).

---

## 3. Responsible Disclosure Process

1. **Initial Acknowledgment:** Within **48 hours**, our security team will acknowledge receipt of your vulnerability report.
2. **Assessment & Validation:** We will reproduce and assess the severity using CVSS v3.1 scoring within **5 business days**.
3. **Patch Development:** A private patch will be authored and tested against our automated security suite (`cargo test --workspace`).
4. **Coordinated Disclosure:** We will publish a patched release and credit the reporter in release notes and CVE advisories (unless anonymity is requested).

---

## 4. Security Philosophy & Zero Trust Architecture

SamuSoporte is built under Zero Trust and Zero-Knowledge principles:
- **Noise Protocol Framework:** All sessions establish end-to-end encryption (`Noise_XX_25519_ChaChaPoly_BLAKE2s`).
- **Relay Servers:** The relay server acts solely as a blind packet transit router and cannot decrypt video streams, keystrokes, or files.
- **Short Authentication String (SAS):** Anti-Man-in-the-Middle mutual verification displayed on both host and guest screens.
- **Memory Hygiene:** Secret keys implement `Zeroize` to scrub sensitive key material from RAM when dropped.
