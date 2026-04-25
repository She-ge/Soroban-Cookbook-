# 🛠️ Utility Scripts

This directory contains a suite of automation scripts designed to streamline the Soroban smart contract development lifecycle. These utilities handle building, testing, and deploying contracts with consistent patterns and robust error handling.

---

## 🏗️ build.sh

The `build.sh` script compiles Soroban contracts into optimized WebAssembly (WASM) files suitable for deployment.

### 📋 Parameters

| Parameter | Type | Required | Description | Default |
|-----------|------|----------|-------------|---------|
| `example-path` | String | No | Relative path to a contract directory (e.g., `examples/basics/01-hello-world`) | All examples |

### 🚀 Usage Examples

```bash
# Build a specific contract
./scripts/build.sh examples/basics/01-hello-world

# Build all contracts in the repository
./scripts/build.sh
```

### 💡 Common Use Cases

*   **Development Iteration**: Run for a single contract to quickly verify it compiles after changes.
*   **Final Release Prep**: Build all contracts to check total binary sizes and ensure workspace-wide consistency.
*   **CI/CD Pipeline**: Integrate into automated workflows to generate artifacts for deployment.

---

## 🧪 test.sh

A comprehensive testing utility that manages unit tests, code linting (Clippy), formatting checks, and coverage reports.

### 📋 Parameters

| Flag | Long Form | Description |
|------|-----------|-------------|
| `-v` | `--verbose` | Shows detailed test output and stdout from tests |
| `-c` | `--clippy` | Runs the Clippy linter to find common mistakes and improve code quality |
| `-f` | `--format` | Checks if the code adheres to the standard Rust formatting |
| `-a` | `--all` | Runs all checks: Unit tests, Clippy, and Formatting check |
| `--coverage`| N/A | Generates a code coverage report using `cargo-tarpaulin` |
| `-h` | `--help` | Displays the help message |

> [!NOTE]
> You can also provide an optional positional argument `<example-path>` to target a specific contract or category.

### 🚀 Usage Examples

```bash
# Run tests for all examples (quiet mode)
./scripts/test.sh

# Run all checks (test + lintish) for a specific example
./scripts/test.sh -a examples/basics/02-auth

# Generate a coverage report for the entire workspace
./scripts/test.sh --coverage
```

### 💡 Common Use Cases

*   **Pre-Commit Validation**: Use `./scripts/test.sh -a` to ensure code is clean and bug-free before pushing.
*   **Feature Coverage**: Run with `--coverage` after adding a new contract to ensure all logic paths are exercised.
*   **Debugging**: Use `-v` to see `println!` output from your smart contract unit tests.

---

## 🚀 deploy.sh

Automates the deployment process, including building, network verification, and account funding (on testnet).

### 📋 Parameters

| Parameter | Position | Required | Description | Default |
|-----------|----------|----------|-------------|---------|
| `contract-path`| 1 | Yes | Path to the contract directory | N/A |
| `network` | 2 | Yes | Target network (`testnet` or `mainnet`) | N/A |
| `identity` | 3 | No | Secret key identity name from Soroban CLI | `default` |

### 🚀 Usage Examples

```bash
# Deploy to testnet using the alice identity
./scripts/deploy.sh examples/basics/01-hello-world testnet alice

# Deploy to mainnet (requires pre-configured mainnet network and keys)
./scripts/deploy.sh examples/basics/01-hello-world mainnet my-prod-key
```

### 💡 Common Use Cases

*   **Local testing staging**: Deploy to `testnet` to verify contract behavior in a real-world environment before mainnet.
*   **New Developer Onboarding**: Allows new team members to deploy examples without needing to remember long Soroban CLI flags.

---

## 🔧 Prerequisites

Ensure you have the following installed before running these scripts:

- **Rust & Cargo**: [rustup.rs](https://rustup.rs/)
- **WASM Target**: `rustup target add wasm32-unknown-unknown`
- **Soroban CLI**: `cargo install --locked soroban-cli`
- **Tarpaulin** (Optional, for coverage): `cargo install cargo-tarpaulin`

## 📝 Troubleshooting

1.  **Permission Denied**: If you get a "permission denied" error, make the scripts executable:
    ```bash
    chmod +x scripts/*.sh
    ```
2.  **Network Not Found**: Ensure you have added the network to your Soroban CLI config:
    ```bash
    soroban network add --global testnet \
      --rpc-url https://soroban-testnet.stellar.org:443 \
      --network-passphrase "Test SDF Network ; September 2015"
    ```

---

*Part of the [Soroban Cookbook](https://github.com/Soroban-Cookbook/Soroban-Cookbook-) - Streamlining Stellar Smart Contract Development.*
