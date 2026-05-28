# SO4 Config Key Catalog

Every data_store configuration key used in the protocol is listed here.
All keys are derived by `libs/keys/src/lib.rs` using sha256 with length-prefixed components.

---

## Notation

- **Type** — the Rust primitive stored in data_store (via `set_u128`, `set_i128`, `set_bool`, `set_address`).
- **Precision** — unit of the stored value. `FLOAT_PRECISION = 10^30`; `TOKEN_PRECISION = 10^7`.
- **Default** — value if the key has never been set. `0` is returned by all `get_*` calls when the key is absent.
- **Bounds** — valid operating range. Values outside this range may cause math overflows or silently incorrect results.
- **Consumed by** — functions that read this key.

---

## Market keys

These keys are set by `market_factory` at market creation time.

### `MARKET_INDEX_TOKEN` · sha256("MARKET_INDEX_TOKEN" ‖ market)

| Attribute | Value |
|---|---|
| Type | `Address` |
| Default | None (must be set before any trading) |
| Consumed by | `load_market_props` in every handler; `reader::get_market` |

The synthetic index token whose oracle price drives PnL and price impact for this market.

---

### `MARKET_LONG_TOKEN` · sha256("MARKET_LONG_TOKEN" ‖ market)

| Attribute | Value |
|---|---|
| Type | `Address` |
| Default | None |
| Consumed by | `load_market_props` in every handler; `reader::get_market` |

The token used as long-side collateral and held in the pool for long deposits.

---

### `MARKET_SHORT_TOKEN` · sha256("MARKET_SHORT_TOKEN" ‖ market)

| Attribute | Value |
|---|---|
| Type | `Address` |
| Default | None |
| Consumed by | `load_market_props` in every handler; `reader::get_market` |

The token used as short-side collateral and held in the pool for short deposits.

---

## Pool / OI size limits

### `MAX_POOL_AMOUNT` · sha256("MAX_POOL_AMOUNT" ‖ market ‖ token)

| Attribute | Value |
|---|---|
| Type | `u128` |
| Precision | TOKEN_PRECISION (7 decimals) |
| Default | `0` (uncapped when 0) |
| Bounds | `0 .. i128::MAX` |
| Consumed by | `market_utils::validate_pool_amount` |

Maximum token amount the pool for `(market, token)` may hold after a deposit. Set to 0 to disable the cap.

---

### `MAX_OPEN_INTEREST` · sha256("MAX_OPEN_INTEREST" ‖ market ‖ is_long)

| Attribute | Value |
|---|---|
| Type | `u128` |
| Precision | FLOAT_PRECISION (USD) |
| Default | `0` (uncapped when 0) |
| Bounds | `0 .. i128::MAX` |
| Consumed by | `market_utils::validate_open_interest` |

Maximum open interest in USD for one side of a market. Set to 0 to disable the cap.

---

## Position risk parameters

### `MIN_COLLATERAL_FACTOR` · sha256("MIN_COLLATERAL_FACTOR" ‖ market)

| Attribute | Value |
|---|---|
| Type | `u128` |
| Precision | FLOAT_PRECISION (ratio) |
| Default | `0` |
| Bounds | `> 0`; typical range `0.01 × 10^30 .. 0.1 × 10^30` (1 %–10 %) |
| Consumed by | `position_utils::validate_position`, `position_utils::is_liquidatable` |

Minimum collateral as a fraction of position size in USD. A position whose collateral falls below this fraction is liquidatable.

---

### `MAX_LEVERAGE` · sha256("MAX_LEVERAGE" ‖ market)

| Attribute | Value |
|---|---|
| Type | `u128` |
| Precision | FLOAT_PRECISION (ratio) |
| Default | `0` (no limit when 0) |
| Bounds | `> 0`; typical range `10 × 10^30 .. 100 × 10^30` (10×–100×) |
| Consumed by | `position_utils::validate_position` |

Maximum leverage a position may use. Checked on increase and update. Set to 0 to disable.

---

## Fee factors

### `POSITION_FEE_FACTOR` · sha256("POSITION_FEE_FACTOR" ‖ market ‖ for_positive_impact)

| Attribute | Value |
|---|---|
| Type | `u128` |
| Precision | FLOAT_PRECISION (ratio, applied to `size_delta_usd`) |
| Default | `0` |
| Bounds | `0 .. 0.01 × 10^30` (0 %–1 %) |
| Consumed by | `pricing_utils::get_position_fees` |

Fee charged on opening or closing a position. Two values: one for trades that improve pool balance (`for_positive_impact = true`) and one for those that worsen it.

---

### `SWAP_FEE_FACTOR` · sha256("SWAP_FEE_FACTOR" ‖ market ‖ for_positive_impact)

| Attribute | Value |
|---|---|
| Type | `u128` |
| Precision | FLOAT_PRECISION (ratio, applied to `amount_usd`) |
| Default | `0` |
| Bounds | `0 .. 0.01 × 10^30` (0 %–1 %) |
| Consumed by | `pricing_utils::get_swap_fee_amount` |

Fee charged on token swaps routed through this market's pool.

---

## Borrowing (borrow fee)

### `BORROWING_FACTOR` · sha256("BORROWING_FACTOR" ‖ market ‖ is_long)

| Attribute | Value |
|---|---|
| Type | `u128` |
| Precision | FLOAT_PRECISION per second (annual rate / seconds-per-year) |
| Default | `0` |
| Bounds | `0 .. ~3.17 × 10^21` (≈ 0.001 % per second, ~31.5 % per year) |
| Consumed by | `market_utils::update_cumulative_borrowing_factor` |

Base rate multiplied by pool utilisation to produce the per-second borrowing cost.

---

### `BORROWING_EXPONENT_FACTOR` · sha256("BORROWING_EXPONENT_FACTOR" ‖ market ‖ is_long)

| Attribute | Value |
|---|---|
| Type | `u128` |
| Precision | FLOAT_PRECISION |
| Default | `0` |
| Bounds | `1 × 10^30 .. 3 × 10^30` (exponents 1–3) |
| Consumed by | `market_utils::update_cumulative_borrowing_factor` |

Exponent applied to utilisation before multiplying by `BORROWING_FACTOR`. Use `1 × FLOAT_PRECISION` for linear scaling.

---

## Funding rate

### `FUNDING_FACTOR` · sha256("FUNDING_FACTOR" ‖ market)

| Attribute | Value |
|---|---|
| Type | `u128` |
| Precision | FLOAT_PRECISION per second |
| Default | `0` |
| Bounds | `0 .. ~3.17 × 10^21` |
| Consumed by | `market_utils::compute_next_funding_factor` |

Base rate for the funding velocity calculation.

---

### `FUNDING_EXPONENT_FACTOR` · sha256("FUNDING_EXPONENT_FACTOR" ‖ market)

| Attribute | Value |
|---|---|
| Type | `u128` |
| Precision | FLOAT_PRECISION |
| Default | `0` |
| Bounds | `1 × 10^30 .. 3 × 10^30` |
| Consumed by | `market_utils::compute_next_funding_factor` |

Exponent on the OI imbalance term in funding rate computation.

---

### `MIN_FUNDING_FACTOR_PER_SECOND` · sha256("MIN_FUNDING_FACTOR_PER_SECOND" ‖ market)

| Attribute | Value |
|---|---|
| Type | `u128` |
| Precision | FLOAT_PRECISION per second |
| Default | `0` |
| Bounds | `0 .. MAX_FUNDING_FACTOR_PER_SECOND` |
| Consumed by | `market_utils::compute_next_funding_factor` |

Floor for the saved funding factor. Prevents the funding rate from going to zero.

---

### `MAX_FUNDING_FACTOR_PER_SECOND` · sha256("MAX_FUNDING_FACTOR_PER_SECOND" ‖ market)

| Attribute | Value |
|---|---|
| Type | `u128` |
| Precision | FLOAT_PRECISION per second |
| Default | `0` |
| Bounds | `>= MIN_FUNDING_FACTOR_PER_SECOND` |
| Consumed by | `market_utils::compute_next_funding_factor` |

Ceiling for the saved funding factor. Prevents unbounded funding rates.

---

### `FUNDING_INCREASE_FACTOR_PER_SECOND` · sha256("FUNDING_INCREASE_FACTOR_PER_SECOND" ‖ market)

| Attribute | Value |
|---|---|
| Type | `u128` |
| Precision | FLOAT_PRECISION per second |
| Default | `0` |
| Bounds | `0 .. FUNDING_DECREASE_FACTOR_PER_SECOND` |
| Consumed by | `market_utils::compute_next_funding_factor` |

Rate at which the funding factor ramps up when OI is imbalanced.

---

### `FUNDING_DECREASE_FACTOR_PER_SECOND` · sha256("FUNDING_DECREASE_FACTOR_PER_SECOND" ‖ market)

| Attribute | Value |
|---|---|
| Type | `u128` |
| Precision | FLOAT_PRECISION per second |
| Default | `0` |
| Bounds | `>= FUNDING_INCREASE_FACTOR_PER_SECOND` |
| Consumed by | `market_utils::compute_next_funding_factor` |

Rate at which the funding factor ramps down when OI balance improves.

---

## Price impact — positions

### `POSITION_IMPACT_FACTOR` · sha256("POSITION_IMPACT_FACTOR" ‖ market ‖ is_positive)

| Attribute | Value |
|---|---|
| Type | `u128` |
| Precision | FLOAT_PRECISION |
| Default | `0` |
| Bounds | `0 .. 0.01 × 10^30` (positive and negative factors may differ) |
| Consumed by | `pricing_utils::get_position_price_impact` |

Coefficient in the price impact formula: `impact = factor × (Δimbalance ^ exponent)`. Two values: one for impact-improving trades (`is_positive = true`), one for impact-worsening trades.

---

### `POSITION_IMPACT_EXPONENT_FACTOR` · sha256("POSITION_IMPACT_EXPONENT_FACTOR" ‖ market)

| Attribute | Value |
|---|---|
| Type | `u128` |
| Precision | FLOAT_PRECISION |
| Default | `0` |
| Bounds | `1 × 10^30 .. 3 × 10^30` |
| Consumed by | `pricing_utils::get_position_price_impact` |

Exponent on the imbalance delta in the price impact formula.

---

## Price impact — swaps

### `SWAP_IMPACT_FACTOR` · sha256("SWAP_IMPACT_FACTOR" ‖ market ‖ is_positive)

| Attribute | Value |
|---|---|
| Type | `u128` |
| Precision | FLOAT_PRECISION |
| Default | `0` |
| Bounds | `0 .. 0.01 × 10^30` |
| Consumed by | `pricing_utils::get_swap_price_impact` |

Coefficient for swap price impact. Same formula as position impact, but separate values for LP balance-improving versus balance-worsening swaps.

---

### `SWAP_IMPACT_EXPONENT_FACTOR` · sha256("SWAP_IMPACT_EXPONENT_FACTOR" ‖ market)

| Attribute | Value |
|---|---|
| Type | `u128` |
| Precision | FLOAT_PRECISION |
| Default | `0` |
| Bounds | `1 × 10^30 .. 3 × 10^30` |
| Consumed by | `pricing_utils::get_swap_price_impact` |

Exponent on the imbalance delta in the swap price impact formula.

---

## PnL factor caps

### `MAX_PNL_FACTOR` · sha256("MAX_PNL_FACTOR" ‖ pnl_factor_type ‖ market ‖ is_long)

| Attribute | Value |
|---|---|
| Type | `u128` |
| Precision | FLOAT_PRECISION (ratio of pool value) |
| Default | `0` (uncapped when 0) |
| Bounds | `0 .. 1 × 10^30` (0 %–100 %) |
| Consumed by | `market_utils::get_pool_value` |

Maximum ratio of trader PnL to pool value for a given `pnl_factor_type` and side. Three types exist:

| Type key | When applied |
|---|---|
| `MAX_PNL_FACTOR_FOR_TRADERS` | Pool value calculation for the `Reader` |
| `MAX_PNL_FACTOR_FOR_DEPOSITS` | Pool value for deposit price computation |
| `MAX_PNL_FACTOR_FOR_WITHDRAWALS` | Pool value for withdrawal price computation |

---

### `MAX_PNL_FACTOR_FOR_ADL` · sha256("MAX_PNL_FACTOR_FOR_ADL" ‖ market ‖ is_long)

| Attribute | Value |
|---|---|
| Type | `u128` |
| Precision | FLOAT_PRECISION (ratio) |
| Default | `0` |
| Bounds | `0 .. 1 × 10^30` |
| Consumed by | `adl_handler::execute` |

PnL-to-pool ratio above which ADL is permitted for a side. When exceeded the ADL keeper may partially close profitable positions.

---

## ADL flag

### `IS_ADL_ENABLED` · sha256("IS_ADL_ENABLED" ‖ market ‖ is_long)

| Attribute | Value |
|---|---|
| Type | `bool` |
| Default | `false` |
| Consumed by | `adl_handler::execute` |

Runtime toggle that must be set to `true` by an ADL keeper before ADL executions are accepted for this market side.

---

## Deposit guard

### `MIN_MARKET_TOKENS_FOR_FIRST_DEPOSIT` · sha256("MIN_MARKET_TOKENS_FOR_FIRST_DEPOSIT" ‖ market)

| Attribute | Value |
|---|---|
| Type | `u128` |
| Precision | TOKEN_PRECISION |
| Default | `0` |
| Bounds | `0 .. ~10^14` |
| Consumed by | `deposit_handler::execute_deposit` |

Minimum LP tokens that must be minted on the very first deposit to a new (empty) pool. Prevents dust initialisation attacks. Set to 0 to allow any first-deposit amount.

---

## Oracle / token settings

### `STABLE_PRICE` · sha256("STABLE_PRICE" ‖ token)

| Attribute | Value |
|---|---|
| Type | `u128` |
| Precision | FLOAT_PRECISION (USD per token, TOKEN_PRECISION units) |
| Default | `0` (not a stablecoin when 0) |
| Consumed by | `oracle` (overrides oracle spread for pegged assets) |

Fixed USD price for stablecoins. When set, the oracle returns this value instead of the keeper-fed min/max spread.

---

### `TOKEN_DECIMALS` · sha256("TOKEN_DECIMALS" ‖ token)

| Attribute | Value |
|---|---|
| Type | `u128` |
| Precision | integer (number of decimal places) |
| Default | `7` (Stellar standard) |
| Bounds | `0 .. 18` |
| Consumed by | `market_utils` (USD conversion) |

Token decimal precision. Used when converting raw token amounts to USD values. All Stellar SEP-41 tokens use 7 decimals.

---

### `MAX_SWAP_PATH_LENGTH` · sha256("MAX_SWAP_PATH_LENGTH")

| Attribute | Value |
|---|---|
| Type | `u128` |
| Precision | integer (hop count) |
| Default | `0` (uncapped when 0; exchange_router enforces its own 3-hop default) |
| Bounds | `1 .. 10` |
| Consumed by | `exchange_router`, `swap_utils::swap_with_path` |

Maximum number of markets in a swap path. Limits gas / instruction budget per swap.

---

## Fee receiver settings

### `UI_FEE_FACTOR` · sha256("UI_FEE_FACTOR" ‖ ui_fee_receiver)

| Attribute | Value |
|---|---|
| Type | `u128` |
| Precision | FLOAT_PRECISION (ratio of position/swap fee) |
| Default | `0` |
| Bounds | `0 .. 0.5 × 10^30` (max 50 % of protocol fee) |
| Consumed by | `pricing_utils::get_position_fees`, `pricing_utils::get_swap_fee_amount` |

UI integration fee factor. When a ui_fee_receiver is supplied on an order, this fraction of the protocol fee is routed to the UI provider.

---

## Keeper public keys

### `KEEPER_PUBLIC_KEY` prefix · sha256("KEEPER_PUBLIC_KEY")

| Attribute | Value |
|---|---|
| Type | `BytesN<32>` (value is a 32-byte ed25519 public key) |
| Default | None |
| Consumed by | `oracle::set_prices` (ed25519 price signature verification) |

The oracle contract stores each approved keeper's ed25519 public key under a key derived from this prefix. Only prices signed by a registered keeper pass signature verification.

---

## Wasm hash

### `MARKET_TOKEN_WASM_HASH` · sha256("MARKET_TOKEN_WASM_HASH")

| Attribute | Value |
|---|---|
| Type | `BytesN<32>` |
| Default | None (market creation fails without this) |
| Consumed by | `market_factory::create_market` |

Wasm hash of the `market_token` contract binary. The market factory deploys a new `market_token` instance per market using this hash.

---

## Referral keys

### `REFERRAL_CODE` · sha256("REFERRAL_CODE" ‖ account)

| Attribute | Value |
|---|---|
| Type | `BytesN<32>` |
| Default | None |
| Consumed by | `referral_storage::set_trader_referral_code`, `pricing_utils::get_position_fees` |

The referral code hash an account has registered as a referee. Used to look up the referrer's tier and apply fee discounts.

---

### `REFERRER` · sha256("REFERRER" ‖ code)

| Attribute | Value |
|---|---|
| Type | `Address` |
| Default | None |
| Consumed by | `referral_storage`, `pricing_utils::get_position_fees` |

Maps a referral code hash to the referrer's address. Used to compute rebates.

---

## Configuration checklist for testnet deployment

When initialising a new market, set the following keys (via the data_store controller) before accepting user transactions:

1. `MARKET_INDEX_TOKEN` / `MARKET_LONG_TOKEN` / `MARKET_SHORT_TOKEN` — set by `market_factory::create_market`.
2. `MAX_POOL_AMOUNT` (long and short) — start with a conservative cap.
3. `MAX_OPEN_INTEREST` (long and short) — set to a fraction of pool TVL.
4. `MIN_COLLATERAL_FACTOR` — e.g. `0.01 × FLOAT_PRECISION` (1 %).
5. `MAX_LEVERAGE` — e.g. `50 × FLOAT_PRECISION` (50×).
6. `POSITION_FEE_FACTOR` (positive and negative impact) — e.g. `0.001 × FLOAT_PRECISION` (0.1 %).
7. `SWAP_FEE_FACTOR` (positive and negative impact) — e.g. `0.0003 × FLOAT_PRECISION` (0.03 %).
8. `BORROWING_FACTOR` (long and short) — e.g. `3.17 × 10^19` ≈ 0.001 % per second.
9. `BORROWING_EXPONENT_FACTOR` (long and short) — e.g. `1 × FLOAT_PRECISION`.
10. `FUNDING_FACTOR`, `FUNDING_EXPONENT_FACTOR`, `MIN/MAX_FUNDING_FACTOR_PER_SECOND`, `FUNDING_INCREASE/DECREASE_FACTOR_PER_SECOND`.
11. `POSITION_IMPACT_FACTOR` / `POSITION_IMPACT_EXPONENT_FACTOR` — size and curvature of position price impact.
12. `SWAP_IMPACT_FACTOR` / `SWAP_IMPACT_EXPONENT_FACTOR` — size and curvature of swap price impact.
13. `MAX_PNL_FACTOR` for all three types (traders, deposits, withdrawals), long and short.
14. `MAX_PNL_FACTOR_FOR_ADL` (long and short) — e.g. `0.45 × FLOAT_PRECISION`.
15. `TOKEN_DECIMALS` for each token (if not Stellar standard 7).
16. `MARKET_TOKEN_WASM_HASH` — set via `market_factory::set_market_token_wasm_hash` before first market deployment.
