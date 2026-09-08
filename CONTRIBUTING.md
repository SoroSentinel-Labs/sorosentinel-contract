# Contributing to sorosentinel-contract

## Setup
```
cargo build --target wasm32-unknown-unknown --release
cargo test
cargo clippy --all-targets
```

## Before opening a PR
- Run the commands above and make sure they pass.
- Keep the PR scoped to one issue; reference it with `Closes #N`.
- Update README.md if you changed public behavior.

## Related repos
- sorosentinel-backend — scores addresses and calls this contract
- sorosentinel-frontend — dashboard reading data derived from this contract
