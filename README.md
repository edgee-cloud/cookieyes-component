<div align="center">
<p align="center">
  <a href="https://www.edgee.cloud">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://cdn.edgee.cloud/img/component-dark.svg">
      <img src="https://cdn.edgee.cloud/img/component.svg" height="100" alt="Edgee">
    </picture>
  </a>
</p>
</div>

<h1 align="center">CookieYes component for Edgee</h1>

[![Coverage Status](https://coveralls.io/repos/github/edgee-cloud/cookieyes-component/badge.svg)](https://coveralls.io/github/edgee-cloud/cookieyes-component)
[![GitHub issues](https://img.shields.io/github/issues/edgee-cloud/cookieyes-component.svg)](https://github.com/edgee-cloud/cookieyes-component/issues)
[![Edgee Component Registry](https://img.shields.io/badge/Edgee_Component_Registry-Public-green.svg)](https://www.edgee.cloud/edgee/cookieyes-consent-mapping)

This component enables seamless integration between [Edgee](https://www.edgee.cloud) and [CookieYes](https://www.cookieyes.com), allowing you to use CookieYes as the Consent Management Platform on Edgee.

## Quick Start

1. Download the latest component version from our [releases page](../../releases)
2. Place the `cookieyes.wasm` file in your server (e.g., `/var/edgee/components`)
3. Add the following configuration to your `edgee.toml`:

```toml
[[destinations.consent_management]]
id = "cookieyes"
file = "/var/edgee/components/cookieyes.wasm"
```

## Development

### Building from Source
Prerequisites:
- [Rust](https://www.rust-lang.org/tools/install)
- [Edgee CLI](https://github.com/edgee-cloud/edgee)

Build command:
```bash
edgee components build
```

Test commands:
```bash
edgee components test
cargo test
```

Test coverage command:
```bash
cargo llvm-cov --all-features
```

### Security
Report security vulnerabilities to [security@edgee.cloud](mailto:security@edgee.cloud)
