# sorosentinel-contract

Soroban smart contract for SoroSentinel — an on-chain risk registry that
authorized off-chain AI agents can write anomaly flags to.

## Status
Early scaffold. `initialize`, `authorize_agent`, `is_agent`, `flag_anomaly`,
and `get_threshold` are implemented. See open issues for what's missing
(role separation, flag history, upgrade safety, tests, CI).

## Build
```
cargo build --target wasm32-unknown-unknown --release
```

## Test
```
cargo test
```
