# Packer Plugin

[![ci](https://github.com/fluentci-io/packer-plugin/actions/workflows/ci.yml/badge.svg)](https://github.com/fluentci-io/packer-plugin/actions/workflows/ci.yml)

This plugin sets up your CI/CD pipeline with a specific version of [packer](https://www.packer.io/).

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm packer setup
```

## Functions

| Name         | Description                                          |
| ------------ | ---------------------------------------------------- |
| setup        | Installs a specific version of packer.               |
| build        | Build image(s) from template                         |
| fix          | Fixes templates from old versions of packer          |
| fmt          | Rewrites HCL2 config files to canonical format       |
| hcl2_upgrade | Transform a JSON template into an HCL2 configuration |
| validate     | Check that a template is valid                       |

## Code Usage

Add `fluentci-pdk` crate to your `Cargo.toml`:

```toml
[dependencies]
fluentci-pdk = "0.1.9"
```

Use the following code to call the plugin:

```rust
use fluentci_pdk::dag;

// ...

dag().call("https://pkg.fluentci.io/packer@v0.1.0?wasm=1", "setup", vec!["latest"])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: packer
    args: |
      setup
- name: Show packer version
  run: |
    type packer
    packer validate
```
