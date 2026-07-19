# Security policy

Grafik is experimental visualization software and is not production ready.

## Report a vulnerability

Do not open a public issue for a suspected vulnerability. Use GitHub's private vulnerability
reporting for `kapsel-cloud/grafik`. Include affected revision, reproduction steps, impact, and any
suggested mitigation. Maintainers will acknowledge a complete report when available; no response or
remediation SLA is promised during bootstrap.

## Security boundary

The browser tracer is simulated and performs no infrastructure work. Grafik must never embed
infrastructure credentials, product signing keys, remote authority, or other operational secrets
in browser assets. The Rust/WASM package manipulates simulation data only and does not access the
DOM, network, filesystem, or environment.

Dependencies and generated WASM bindings remain part of the review surface. A successful build or
deterministic replay is not a security audit.
