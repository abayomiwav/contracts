# Build Guide & WASM Size Budget

**Issue:** [#301](https://github.com/SO4-Markets/contracts/issues/301)

## Prerequisites

```bash
# Install the wasm32 target
rustup target add wasm32-unknown-unknown

# Install wasm-opt (part of the Binaryen toolchain)
# macOS
brew install binaryen

# Debian/Ubuntu
apt-get install binaryen
```

## Building contracts

```bash
# Build all contracts for release
cargo build --target wasm32-unknown-unknown --release

# Optimise with wasm-opt (required step before measuring size or deploying)
for f in target/wasm32-unknown-unknown/release/*.wasm; do
  wasm-opt -O3 -o "$f" "$f"
done
```

## WASM size baseline

> Baseline measured after `wasm-opt -O3` on commit **ed754df**.  
> Re-run the CI workflow on every PR to track changes against this table.

| Contract | Optimised WASM size (bytes) | Notes |
|---|---|---|
| `data_store` | — | Run `make wasm-sizes` to populate |
| `oracle` | — | |
| `role_store` | — | |
| `market_factory` | — | |
| `market_token` | — | |
| `deposit_handler` | — | |
| `deposit_vault` | — | |
| `withdrawal_handler` | — | |
| `withdrawal_vault` | — | |
| `order_handler` | — | Largest contract; monitor closely |
| `order_vault` | — | |
| `order_cleanup` | — | |
| `fee_handler` | — | |
| `fee_batch_sweeper` | — | |
| `liquidation_handler` | — | |
| `adl_handler` | — | |
| `reader` | — | |
| `market_util_reader` | — | |
| `referral_storage` | — | |
| `insurance_fund_router` | — | |
| `exchange_router` | — | |
| `test_faucet` | — | Test-only; excluded from budget |
| `test_token` | — | Test-only; excluded from budget |

Run `make wasm-sizes` (see `Makefile`) to fill in the table and update the baseline file.

## Size budget rules

| Threshold | Action |
|---|---|
| < +5% growth vs baseline | Pass silently |
| +5% – +10% growth | PR author receives a warning comment; merge is not blocked |
| > +10% growth | CI step fails; PR cannot merge until size is reduced or the baseline is intentionally updated |

## Updating the baseline

If a size increase is intentional (e.g. a significant new feature), update `docs/build-baseline.json` in the same PR:

```bash
make wasm-baseline   # regenerates docs/build-baseline.json from current build
```

Then commit the updated baseline alongside the feature change.

## Makefile targets

```makefile
wasm-sizes:
	@echo "Contract\tSize (bytes)"
	@for f in target/wasm32-unknown-unknown/release/*.wasm; do \
	  name=$$(basename $$f .wasm); \
	  size=$$(wc -c < $$f); \
	  echo "$$name\t$$size"; \
	done

wasm-baseline:
	@cargo build --target wasm32-unknown-unknown --release 2>/dev/null
	@for f in target/wasm32-unknown-unknown/release/*.wasm; do \
	  wasm-opt -O3 -o "$$f" "$$f"; \
	done
	@python3 scripts/gen_baseline.py > docs/build-baseline.json
```
